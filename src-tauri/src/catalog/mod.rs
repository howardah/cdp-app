//! Catalog boundary shared by the runtime commands.
//! Manifests are data, never executable command fragments.
pub mod types;

use std::collections::HashSet;
use types::ProcessDefinition;

/// Embed the reviewed manifests so catalog behavior is independent of cwd or
/// user-controlled files.
pub fn bundled_catalog() -> Result<Vec<ProcessDefinition>, String> {
    load_catalog(&[
        include_str!("../../../src/processes/definitions/modify-speed.json"),
        include_str!("../../../src/processes/definitions/modify-loudness.json"),
        include_str!("../../../src/processes/definitions/sfedit-join.json"),
        include_str!("../../../src/processes/definitions/pvoc-analyze.json"),
        include_str!("../../../src/processes/definitions/pvoc-synthesize.json"),
        include_str!("../../../src/processes/definitions/isolate.json"),
        include_str!("../../../src/processes/definitions/sfedit-cut.json"),
        include_str!("../../../src/processes/definitions/sfedit-cutend.json"),
        include_str!("../../../src/processes/definitions/sfedit-excise.json"),
        include_str!("../../../src/processes/definitions/sfedit-insil.json"),
        include_str!("../../../src/processes/definitions/sfedit-insert.json"),
        include_str!("../../../src/processes/definitions/sfedit-replace.json"),
        include_str!("../../../src/processes/definitions/sfedit-cutmany.json"),
        include_str!("../../../src/processes/definitions/sfedit-masks.json"),
        include_str!("../../../src/processes/definitions/modify-space.json"),
        include_str!("../../../src/processes/definitions/modify-revecho.json"),
        include_str!("../../../src/processes/definitions/sfedit-zcut.json"),
        include_str!("../../../src/processes/definitions/sfedit-zcuts.json"),
        include_str!("../../../src/processes/definitions/sfedit-excises.json"),
        include_str!("../../../src/processes/definitions/sfedit-noisecut.json"),
        include_str!("../../../src/processes/definitions/modify-radical.json"),
        include_str!("../../../src/processes/definitions/modify-convolve.json"),
        include_str!("../../../src/processes/definitions/modify-stack.json"),
        include_str!("../../../src/processes/definitions/modify-shudder.json"),
        include_str!("../../../src/processes/definitions/modify-scaledpan.json"),
        include_str!("../../../src/processes/definitions/modify-brassage.json"),
        include_str!("../../../src/processes/definitions/modify-sausage.json"),
        include_str!("../../../src/processes/definitions/modify-spaceform.json"),
        include_str!("../../../src/processes/definitions/modify-findpan.json"),
        include_str!("../../../src/processes/definitions/reverb.json"),
        include_str!("../../../src/processes/definitions/filter-fixed.json"),
        include_str!("../../../src/processes/definitions/filter-lohi.json"),
        include_str!("../../../src/processes/definitions/filter-variable.json"),
        include_str!("../../../src/processes/definitions/filter-sweeping.json"),
        include_str!("../../../src/processes/definitions/filter-phasing.json"),
        include_str!("../../../src/processes/definitions/filter-iterated.json"),
        include_str!("../../../src/processes/definitions/filter-bank.json"),
        include_str!("../../../src/processes/definitions/filter-userbank.json"),
        include_str!("../../../src/processes/definitions/filter-varibank.json"),
        include_str!("../../../src/processes/definitions/phasor.json"),
        include_str!("../../../src/processes/definitions/blur-avrg.json"),
        include_str!("../../../src/processes/definitions/blur-blur.json"),
        include_str!("../../../src/processes/definitions/blur-chorus.json"),
        include_str!("../../../src/processes/definitions/blur-drunk.json"),
        include_str!("../../../src/processes/definitions/blur-noise.json"),
        include_str!("../../../src/processes/definitions/blur-scatter.json"),
        include_str!("../../../src/processes/definitions/blur-spread.json"),
        include_str!("../../../src/processes/definitions/extend-drunk.json"),
        include_str!("../../../src/processes/definitions/extend-repetitions.json"),
        include_str!("../../../src/processes/definitions/extend-scramble.json"),
        include_str!("../../../src/processes/definitions/extend-sequence.json"),
        include_str!("../../../src/processes/definitions/extend-zigzag.json"),
        include_str!("../../../src/processes/definitions/hover.json"),
        include_str!("../../../src/processes/definitions/hover2.json"),
        include_str!("../../../src/processes/definitions/sfecho-echo.json"),
        include_str!("../../../src/processes/definitions/spectstr.json"),
        include_str!("../../../src/processes/definitions/stretch-spectrum.json"),
        include_str!("../../../src/processes/definitions/stretch-time.json"),
        include_str!("../../../src/processes/definitions/submix-balance.json"),
        include_str!("../../../src/processes/definitions/submix-merge.json"),
        include_str!("../../../src/processes/definitions/envel-attack.json"),
        include_str!("../../../src/processes/definitions/envel-curtail.json"),
        include_str!("../../../src/processes/definitions/envel-dovetail.json"),
        include_str!("../../../src/processes/definitions/envel-tremolo.json"),
        include_str!("../../../src/processes/definitions/envel-cyclic.json"),
        include_str!("../../../src/processes/definitions/envel-swell.json"),
        include_str!("../../../src/processes/definitions/envel-pluck.json"),
        include_str!("../../../src/processes/definitions/envel-warp.json"),
        include_str!("../../../src/processes/definitions/envel-impose.json"),
        include_str!("../../../src/processes/definitions/envel-replace.json"),
        include_str!("../../../src/processes/definitions/envel-extract.json"),
        include_str!("../../../src/processes/definitions/envel-create.json"),
        include_str!("../../../src/processes/definitions/envnu-expdecay.json"),
        include_str!("../../../src/processes/definitions/envnu-peakchop.json"),
        include_str!("../../../src/processes/definitions/flatten.json"),
        include_str!("../../../src/processes/definitions/sfedit-joinseq.json"),
        include_str!("../../../src/processes/definitions/sfedit-joindyn.json"),
        include_str!("../../../src/processes/definitions/sfedit-randchunks.json"),
        include_str!("../../../src/processes/definitions/sfedit-twixt.json"),
        include_str!("../../../src/processes/definitions/sfedit-sphinx.json"),
        include_str!("../../../src/processes/definitions/sfedit-syllables.json"),
        include_str!("../../../src/processes/definitions/extend-baktobak.json"),
        include_str!("../../../src/processes/definitions/bounce.json"),
        include_str!("../../../src/processes/definitions/extend-freeze.json"),
        include_str!("../../../src/processes/definitions/extend-iterate.json"),
        include_str!("../../../src/processes/definitions/extend-loop.json"),
        include_str!("../../../src/processes/definitions/envel-brktoenv.json"),
        include_str!("../../../src/processes/definitions/envel-dbtoenv.json"),
        include_str!("../../../src/processes/definitions/extend-doublets.json"),
        include_str!("../../../src/processes/definitions/focus-step.json"),
        include_str!("../../../src/processes/definitions/submix-mergemany.json"),
        include_str!("../../../src/processes/definitions/submix-crossfade.json"),
        include_str!("../../../src/processes/definitions/submix-mix.json"),
        include_str!("../../../src/processes/definitions/submix-interleave.json"),
        include_str!("../../../src/processes/definitions/submix-pan.json"),
        include_str!("../../../src/processes/definitions/submix-spacewarp.json"),
        include_str!("../../../src/processes/definitions/blur-suppress.json"),
        include_str!("../../../src/processes/definitions/blur-weave.json"),
        include_str!("../../../src/processes/definitions/blur-shuffle.json"),
        include_str!("../../../src/processes/definitions/iterline.json"),
        include_str!("../../../src/processes/definitions/iterlinef.json"),
        include_str!("../../../src/processes/definitions/submix-inbetween.json"),
        include_str!("../../../src/processes/definitions/submix-inbetween2.json"),
        include_str!("../../../src/processes/definitions/submix-sync.json"),
        include_str!("../../../src/processes/definitions/submix-syncattack.json"),
        include_str!("../../../src/processes/definitions/submix-timewarp.json"),
        include_str!("../../../src/processes/definitions/submix-faders.json"),
        include_str!("../../../src/processes/definitions/submix-addtomix.json"),
        include_str!("../../../src/processes/definitions/combine-cross.json"),
        include_str!("../../../src/processes/definitions/combine-diff.json"),
        include_str!("../../../src/processes/definitions/combine-interleave.json"),
        include_str!("../../../src/processes/definitions/combine-max.json"),
        include_str!("../../../src/processes/definitions/combine-mean.json"),
        include_str!("../../../src/processes/definitions/combine-sum.json"),
        include_str!("../../../src/processes/definitions/focus-accu.json"),
        include_str!("../../../src/processes/definitions/focus-exag.json"),
        include_str!("../../../src/processes/definitions/focus-focus.json"),
        include_str!("../../../src/processes/definitions/focus-fold.json"),
        include_str!("../../../src/processes/definitions/focus-freeze.json"),
        include_str!("../../../src/processes/definitions/focus-hold.json"),
        include_str!("../../../src/processes/definitions/envel-dbtogain.json"),
        include_str!("../../../src/processes/definitions/envel-envtobrk.json"),
        include_str!("../../../src/processes/definitions/envel-envtodb.json"),
        include_str!("../../../src/processes/definitions/envel-gaintodb.json"),
        include_str!("../../../src/processes/definitions/envel-reshape.json"),
        include_str!("../../../src/processes/definitions/envel-replot.json"),
        include_str!("../../../src/processes/definitions/envel-scaled.json"),
        include_str!("../../../src/processes/definitions/envel-timegrid.json"),
        include_str!("../../../src/processes/definitions/filter-bankfrqs.json"),
        include_str!("../../../src/processes/definitions/filter-vfilters.json"),
        include_str!("../../../src/processes/definitions/submix-atstep.json"),
        include_str!("../../../src/processes/definitions/submix-attenuate.json"),
        include_str!("../../../src/processes/definitions/submix-dummy.json"),
        include_str!("../../../src/processes/definitions/submix-getlevel.json"),
        include_str!("../../../src/processes/definitions/submix-model.json"),
        include_str!("../../../src/processes/definitions/submix-ongrid.json"),
        include_str!("../../../src/processes/definitions/submix-shuffle.json"),
        include_str!("../../../src/processes/definitions/submix-test.json"),
        include_str!("../../../src/processes/definitions/combine-make.json"),
        include_str!("../../../src/processes/definitions/combine-make2.json"),
        include_str!("../../../src/processes/definitions/distort-average.json"),
        include_str!("../../../src/processes/definitions/distort-clip.json"),
        include_str!("../../../src/processes/definitions/distort-cyclecnt.json"),
        include_str!("../../../src/processes/definitions/distort-delete.json"),
        include_str!("../../../src/processes/definitions/distort-divide.json"),
        include_str!("../../../src/processes/definitions/distort-envel.json"),
        include_str!("../../../src/processes/definitions/distort-filter.json"),
        include_str!("../../../src/processes/definitions/distort-fractal.json"),
        include_str!("../../../src/processes/definitions/distort-harmonic.json"),
        include_str!("../../../src/processes/definitions/distort-interact.json"),
        include_str!("../../../src/processes/definitions/distort-interpolate.json"),
        include_str!("../../../src/processes/definitions/distort-multiply.json"),
        include_str!("../../../src/processes/definitions/distort-omit.json"),
        include_str!("../../../src/processes/definitions/distort-overload.json"),
        include_str!("../../../src/processes/definitions/distort-pitch.json"),
        include_str!("../../../src/processes/definitions/distort-pulsed.json"),
        include_str!("../../../src/processes/definitions/distort-reform.json"),
        include_str!("../../../src/processes/definitions/distort-repeat.json"),
        include_str!("../../../src/processes/definitions/distort-repeat2.json"),
        include_str!("../../../src/processes/definitions/distort-replace.json"),
        include_str!("../../../src/processes/definitions/distort-replim.json"),
        include_str!("../../../src/processes/definitions/distort-reverse.json"),
        include_str!("../../../src/processes/definitions/distort-shuffle.json"),
        include_str!("../../../src/processes/definitions/distort-telescope.json"),
        include_str!("../../../src/processes/definitions/grain-align.json"),
        include_str!("../../../src/processes/definitions/grain-assess.json"),
        include_str!("../../../src/processes/definitions/grain-count.json"),
        include_str!("../../../src/processes/definitions/grain-duplicate.json"),
        include_str!("../../../src/processes/definitions/grain-find.json"),
        include_str!("../../../src/processes/definitions/grain-grev.json"),
        include_str!("../../../src/processes/definitions/grain-noise-extend.json"),
        include_str!("../../../src/processes/definitions/grain-omit.json"),
        include_str!("../../../src/processes/definitions/grain-remotif.json"),
        include_str!("../../../src/processes/definitions/grain-reorder.json"),
        include_str!("../../../src/processes/definitions/grain-repitch.json"),
        include_str!("../../../src/processes/definitions/grain-reposition.json"),
        include_str!("../../../src/processes/definitions/grain-rerhythm.json"),
        include_str!("../../../src/processes/definitions/grain-reverse.json"),
        include_str!("../../../src/processes/definitions/grain-r-extend.json"),
        include_str!("../../../src/processes/definitions/grain-timewarp.json"),
        include_str!("../../../src/processes/definitions/repitch-analenv.json"),
        include_str!("../../../src/processes/definitions/repitch-approx.json"),
        include_str!("../../../src/processes/definitions/repitch-combine.json"),
        include_str!("../../../src/processes/definitions/repitch-combineb.json"),
        include_str!("../../../src/processes/definitions/repitch-cut.json"),
        include_str!("../../../src/processes/definitions/repitch-exag.json"),
        include_str!("../../../src/processes/definitions/repitch-fix.json"),
        include_str!("../../../src/processes/definitions/repitch-generate.json"),
        include_str!("../../../src/processes/definitions/repitch-getpitch.json"),
        include_str!("../../../src/processes/definitions/repitch-insertsil.json"),
        include_str!("../../../src/processes/definitions/repitch-insertzeros.json"),
        include_str!("../../../src/processes/definitions/repitch-interp.json"),
        include_str!("../../../src/processes/definitions/repitch-invert.json"),
        include_str!("../../../src/processes/definitions/repitch-noisetosil.json"),
        include_str!("../../../src/processes/definitions/repitch-pchshift.json"),
        include_str!("../../../src/processes/definitions/repitch-pchtotext.json"),
        include_str!("../../../src/processes/definitions/repitch-pitchtosil.json"),
        include_str!("../../../src/processes/definitions/repitch-quantise.json"),
        include_str!("../../../src/processes/definitions/repitch-randomise.json"),
        include_str!("../../../src/processes/definitions/repitch-smooth.json"),
        include_str!("../../../src/processes/definitions/repitch-synth.json"),
        include_str!("../../../src/processes/definitions/repitch-transpose.json"),
        include_str!("../../../src/processes/definitions/repitch-transposef.json"),
        include_str!("../../../src/processes/definitions/repitch-vibrato.json"),
        include_str!("../../../src/processes/definitions/repitch-vowels.json"),
        include_str!("../../../src/processes/definitions/sndinfo-props.json"),
        include_str!("../../../src/processes/definitions/sndinfo-len.json"),
        include_str!("../../../src/processes/definitions/sndinfo-lens.json"),
        include_str!("../../../src/processes/definitions/sndinfo-sumlen.json"),
        include_str!("../../../src/processes/definitions/sndinfo-timediff.json"),
        include_str!("../../../src/processes/definitions/sndinfo-smptime.json"),
        include_str!("../../../src/processes/definitions/sndinfo-timesmp.json"),
        include_str!("../../../src/processes/definitions/sndinfo-maxsamp.json"),
        include_str!("../../../src/processes/definitions/sndinfo-maxsamp2.json"),
        include_str!("../../../src/processes/definitions/sndinfo-loudchan.json"),
        include_str!("../../../src/processes/definitions/sndinfo-findhole.json"),
        include_str!("../../../src/processes/definitions/sndinfo-diff.json"),
        include_str!("../../../src/processes/definitions/sndinfo-chandiff.json"),
        include_str!("../../../src/processes/definitions/brktopi.json"),
        include_str!("../../../src/processes/definitions/ptobrk-withzeros.json"),
    ])
}

pub fn load_catalog(json_documents: &[&str]) -> Result<Vec<ProcessDefinition>, String> {
    let mut ids = HashSet::new();
    let mut catalog = Vec::with_capacity(json_documents.len());
    for document in json_documents {
        let definition: ProcessDefinition = serde_json::from_str(document)
            .map_err(|error| format!("invalid process manifest: {error}"))?;
        if definition.schema_version != 1 {
            return Err(format!("unsupported catalog schema for {}", definition.id));
        }
        if definition.id.trim().is_empty() {
            return Err("process id cannot be empty".into());
        }
        if !ids.insert(definition.id.clone()) {
            return Err(format!("duplicate process id: {}", definition.id));
        }
        if definition.modes.is_empty() {
            return Err(format!("process {} has no modes", definition.id));
        }
        if definition
            .modes
            .iter()
            .any(|mode| mode.id.trim().is_empty())
        {
            return Err(format!("process {} has an empty mode id", definition.id));
        }
        validate_definition(&definition)?;
        catalog.push(definition);
    }
    Ok(catalog)
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .split('-')
            .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_alphanumeric()))
        && id.chars().next().is_some_and(|c| c.is_ascii_lowercase())
}

fn validate_definition(definition: &ProcessDefinition) -> Result<(), String> {
    let mut modes = HashSet::new();
    for mode in &definition.modes {
        if !valid_id(&mode.id) || !modes.insert(&mode.id) {
            return Err(format!(
                "invalid or duplicate mode id in {}: {}",
                definition.id, mode.id
            ));
        }
        let mut inputs = HashSet::new();
        for input in &mode.inputs {
            if !valid_id(&input.id) || !inputs.insert(&input.id) {
                return Err(format!(
                    "invalid or duplicate input id in {}: {}",
                    mode.id, input.id
                ));
            }
            if input.max_items.is_some_and(|max| max < input.min_items) {
                return Err(format!("input {} has maxItems below minItems", input.id));
            }
            for constraint in &input.constraints {
                if let types::FileConstraint::Channels { min, max } = constraint {
                    if min > max {
                        return Err(format!("invalid channel range for {}", input.id));
                    }
                }
            }
        }
        let mut params = HashSet::new();
        for parameter in &mode.parameters {
            let base = parameter_base(parameter);
            if !valid_id(&base.id) || !params.insert(&base.id) {
                return Err(format!(
                    "invalid or duplicate parameter id in {}: {}",
                    mode.id, base.id
                ));
            }
            validate_binding(&base.cli)?;
            let d = &base.details;
            if let (Some(min), Some(max)) = (
                d.get("min").and_then(|v| v.as_f64()),
                d.get("max").and_then(|v| v.as_f64()),
            ) {
                if !min.is_finite() || !max.is_finite() || min > max {
                    return Err(format!("invalid numeric bounds for {}", base.id));
                }
            }
            if let Some(default) = d.get("default") {
                validate_default(parameter, default)?;
            }
        }
        for token in &mode.argument_order {
            match token {
                types::ArgumentToken::Input { input_id } if !inputs.contains(input_id) => {
                    return Err(format!("unknown input reference: {input_id}"))
                }
                types::ArgumentToken::Parameter { parameter_id }
                    if !params.contains(parameter_id) =>
                {
                    return Err(format!("unknown parameter reference: {parameter_id}"))
                }
                types::ArgumentToken::Literal { value } if value.contains('\0') => {
                    return Err("literal contains NUL".into())
                }
                _ => {}
            }
        }
        for constraint in &mode.constraints {
            for reference in constraint_refs(constraint) {
                match reference {
                    types::ValueRef::Parameter { id } if !params.contains(id) => {
                        return Err(format!("unknown constraint parameter: {id}"))
                    }
                    types::ValueRef::InputMetadata { input_id, .. }
                        if !inputs.contains(input_id) =>
                    {
                        return Err(format!("unknown constraint input: {input_id}"))
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn validate_binding(binding: &types::CliBinding) -> Result<(), String> {
    let flag = match binding {
        types::CliBinding::Positional => return Ok(()),
        types::CliBinding::Option { flag, .. } | types::CliBinding::BooleanFlag { flag } => flag,
    };
    if !flag.starts_with('-')
        || flag.len() < 2
        || !flag[1..]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
    {
        return Err(format!("unsafe CLI flag: {flag}"));
    }
    Ok(())
}

fn validate_default(
    parameter: &types::ParameterDefinition,
    value: &serde_json::Value,
) -> Result<(), String> {
    let base = parameter_base(parameter);
    let numeric = |v: &serde_json::Value| v.as_f64().filter(|n| n.is_finite());
    let number = match parameter {
        types::ParameterDefinition::Number(_) | types::ParameterDefinition::Integer(_) => {
            numeric(value)
        }
        types::ParameterDefinition::NumberOrBreakpoint(_) => value.get("value").and_then(numeric),
        _ => None,
    };
    if matches!(
        parameter,
        types::ParameterDefinition::Number(_)
            | types::ParameterDefinition::Integer(_)
            | types::ParameterDefinition::NumberOrBreakpoint(_)
    ) && number.is_none()
    {
        return Err(format!("impossible numeric default for {}", base.id));
    }
    if let Some(n) = number {
        if let Some(min) = base.details.get("min").and_then(|v| v.as_f64()) {
            if n < min {
                return Err(format!("default below minimum for {}", base.id));
            }
        }
        if let Some(max) = base.details.get("max").and_then(|v| v.as_f64()) {
            if n > max {
                return Err(format!("default above maximum for {}", base.id));
            }
        }
        if matches!(parameter, types::ParameterDefinition::Integer(_)) && n.fract() != 0.0 {
            return Err(format!("integer default is fractional for {}", base.id));
        }
    }
    Ok(())
}

fn constraint_refs(constraint: &types::Constraint) -> Vec<&types::ValueRef> {
    match constraint {
        types::Constraint::LessThan { left, right, .. }
        | types::Constraint::LessThanOrEqual { left, right, .. }
        | types::Constraint::GreaterThan { left, right, .. } => vec![left, right],
        types::Constraint::PowerOfTwo { value, .. } => vec![value],
    }
}

fn parameter_base(parameter: &types::ParameterDefinition) -> &types::ParameterBase {
    match parameter {
        types::ParameterDefinition::Number(b)
        | types::ParameterDefinition::Integer(b)
        | types::ParameterDefinition::Choice(b)
        | types::ParameterDefinition::Flag(b)
        | types::ParameterDefinition::File(b)
        | types::ParameterDefinition::NumberOrBreakpoint(b) => b,
    }
}

#[cfg(test)]
mod tests {
    use super::load_catalog;
    #[test]
    fn rejects_duplicate_process_ids() {
        let manifest = r#"{"schemaVersion":1,"id":"x","title":"X","category":"utilities","summary":"x","description":"x","useCases":[],"tags":[],"identity":{"executable":"modify"},"documentation":{"localPath":"x","helpCommand":[],"verifiedWithVersion":"x"},"modes":[{"id":"m","title":"m","summary":"m","inputs":[],"parameters":[],"output":{"kind":"none"},"argumentOrder":[],"constraints":[]}] }"#;
        assert!(load_catalog(&[manifest, manifest]).is_err());
    }
}
