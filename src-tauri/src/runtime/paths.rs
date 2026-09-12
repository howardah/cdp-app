use super::types::InputMetadata;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn suggest_output_path(input: &Path, slug: &str, extension: &str) -> Result<PathBuf, String> {
    let parent = input.parent().ok_or("input has no parent directory")?;
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("input filename is not valid UTF-8")?;
    let safe_slug = slug
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>();
    for n in 1..10000 {
        let suffix = if n == 1 {
            "".to_string()
        } else {
            format!("-{n}")
        };
        let extension = extension.trim_start_matches('.').to_ascii_lowercase();
        if extension.is_empty() || !extension.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err("output extension is invalid".into());
        }
        let path = parent.join(format!("{stem}-{safe_slug}{suffix}.{extension}"));
        if !path.exists() {
            return Ok(path);
        }
    }
    Err("could not find an available output path".into())
}

const MAX_BREAKPOINT_BYTES: u64 = 1024 * 1024;
const MAX_TEXT_BYTES: u64 = 4 * 1024 * 1024;

/// Validate CDP's time/value breakpoint text without interpreting user input as code.
pub fn validate_breakpoint(path: &Path) -> Result<(), String> {
    validate_bounded_text(path, MAX_BREAKPOINT_BYTES, |line, number| {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 2
            || fields
                .iter()
                .any(|v| v.parse::<f64>().map_or(true, |n| !n.is_finite()))
        {
            return Err(format!("invalid breakpoint at line {number}"));
        }
        if fields[0].parse::<f64>().unwrap() < 0.0 {
            return Err(format!(
                "breakpoint time must be non-negative at line {number}"
            ));
        }
        Ok(())
    })
}

pub fn validate_text_data(path: &Path) -> Result<(), String> {
    validate_bounded_text(path, MAX_TEXT_BYTES, |line, number| {
        if line.trim().is_empty() {
            return Ok(());
        }
        if line.chars().any(|c| c.is_control() && c != '\t') {
            return Err(format!("invalid text data at line {number}"));
        }
        Ok(())
    })
}

fn validate_bounded_text<F>(path: &Path, max: u64, mut check: F) -> Result<(), String>
where
    F: FnMut(&str, usize) -> Result<(), String>,
{
    let metadata = std::fs::metadata(path).map_err(|_| "file cannot be inspected".to_string())?;
    if !metadata.is_file() {
        return Err("path is not a regular file".into());
    }
    if metadata.len() > max {
        return Err("file exceeds the supported size limit".into());
    }
    let content =
        std::fs::read_to_string(path).map_err(|_| "file is not valid UTF-8".to_string())?;
    for (line, number) in content.lines().zip(1..) {
        check(line, number)?;
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct SfProps {
    pub duration_seconds: f64,
    pub sample_rate: u32,
    pub channels: u32,
    pub sample_format: String,
}

/// Parse only the stable, labelled fields emitted by sfprops. Unknown labels are ignored,
/// but all required fields must be present and numeric values must be finite and positive.
pub fn parse_sfprops(stdout: &[u8], success: bool) -> Result<SfProps, String> {
    if !success {
        return Err("sfprops failed".into());
    }
    let text =
        std::str::from_utf8(stdout).map_err(|_| "sfprops output is not UTF-8".to_string())?;
    let mut duration = None;
    let mut rate = None;
    let mut channels = None;
    let mut format = None;
    let mut analysis_file = false;
    for line in text.lines() {
        let Some((label, raw)) = line.split_once(':') else {
            continue;
        };
        // Some Release 8 builds print `sample type: : 16-bit`; accept the
        // extra separator while keeping the parser label-driven.
        let value = raw.trim().trim_start_matches(':').trim();
        match label.trim().to_ascii_lowercase().as_str() {
            "duration" | "duration seconds" | "duration (secs)" => {
                duration = value.split_whitespace().next().and_then(|v| v.parse().ok())
            }
            "sample rate" | "samplerate" | "orig rate" => {
                rate = value.split_whitespace().next().and_then(|v| v.parse().ok())
            }
            "channels" | "channel count" => {
                channels = value.split_whitespace().next().and_then(|v| v.parse().ok())
            }
            "sample format" | "sampleformat" | "sample type" if !value.is_empty() => {
                format = Some(value.to_string())
            }
            "file type" if value.to_ascii_lowercase().contains("analysis file") => {
                analysis_file = true;
                format.get_or_insert_with(|| value.to_string());
            }
            "channel format" if analysis_file && !value.is_empty() => {
                format = Some(value.to_string())
            }
            _ => {}
        }
    }
    // Legacy `.ana` files are created from mono sources and sfprops reports
    // spectral-bin count rather than an audio channel count for them.
    if analysis_file && channels.is_none() {
        channels = Some(1);
    }
    let duration = duration
        .filter(|v: &f64| v.is_finite() && *v >= 0.0)
        .ok_or("sfprops output missing duration")?;
    let sample_rate = rate
        .filter(|v| *v > 0)
        .ok_or("sfprops output missing sample rate")?;
    let channels = channels
        .filter(|v| *v > 0)
        .ok_or("sfprops output missing channels")?;
    let sample_format = format.ok_or("sfprops output missing sample format")?;
    Ok(SfProps {
        duration_seconds: duration,
        sample_rate,
        channels,
        sample_format,
    })
}

pub fn inspect_audio_metadata(path: &Path, sfprops: &Path) -> Result<InputMetadata, String> {
    let output = Command::new(sfprops)
        .arg(path)
        .output()
        .map_err(|e| format!("could not inspect input: {e}"))?;
    let props = parse_sfprops(&output.stdout, output.status.success())?;
    Ok(InputMetadata {
        duration_seconds: props.duration_seconds,
        sample_rate: props.sample_rate,
        channels: props.channels,
        sample_format: props.sample_format,
    })
}

pub fn discover_generic_outputs(
    root: &Path,
    extension: &str,
    before: &[PathBuf],
) -> Result<Vec<PathBuf>, String> {
    let parent = root.parent().ok_or("output root has no parent")?;
    let stem = root
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("output root is invalid")?;
    let ext = extension.trim_start_matches('.').to_ascii_lowercase();
    let mut found = Vec::new();
    for entry in std::fs::read_dir(parent).map_err(|_| "output directory cannot be read")? {
        let path = entry.map_err(|_| "output directory cannot be read")?.path();
        if before.contains(&path) || !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|v| v.to_str()) else {
            continue;
        };
        let Some(suffix) = name.strip_prefix(stem) else {
            continue;
        };
        if path
            .extension()
            .and_then(|v| v.to_str())
            .map(|v| v.eq_ignore_ascii_case(&ext))
            != Some(true)
        {
            continue;
        }
        let Some(number_with_ext) = suffix.strip_suffix(&format!(".{ext}")) else {
            continue;
        };
        let number = number_with_ext.strip_prefix('-').unwrap_or(number_with_ext);
        if !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()) {
            found.push(path);
        }
    }
    found.sort();
    Ok(found)
}
