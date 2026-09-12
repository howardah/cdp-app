#!/usr/bin/env bash
set -euo pipefail

# Stage the five CDP executables needed by the Intel macOS MVP. This script is
# deliberately read-only with respect to cdpr8: it copies binaries out of the
# release tree and never patches or rebuilds that tree.
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source_dir="${repo_root}/cdpr8/_cdp/_cdprogs"
destination_dir="${repo_root}/src-tauri/binaries"
target_suffix="-x86_64-apple-darwin"

executables=(modify sfedit pvoc isolate sfprops blur bounce combine envel envnu extend filter flatten focus hover hover2 iterline iterlinef phasor reverb sfecho spectstr stretch submix)

command -v file >/dev/null 2>&1 || {
  printf '%s\n' "stage-cdp-binaries: the 'file' command is required" >&2
  exit 1
}

mkdir -p "${destination_dir}"
for executable in "${executables[@]}"; do
  source_path="${source_dir}/${executable}"
  destination_path="${destination_dir}/${executable}${target_suffix}"

  if [[ ! -f "${source_path}" ]]; then
    printf 'stage-cdp-binaries: missing source: %s\n' "${source_path}" >&2
    exit 1
  fi
  if [[ ! -x "${source_path}" ]]; then
    printf 'stage-cdp-binaries: source is not executable: %s\n' "${source_path}" >&2
    exit 1
  fi
  if ! file -b "${source_path}" | grep -Eq 'Mach-O 64-bit executable x86_64'; then
    printf 'stage-cdp-binaries: source is not an x86_64 Mach-O executable: %s\n' "${source_path}" >&2
    exit 1
  fi

  # cp -p retains the executable mode (and other useful source metadata).
  cp -p "${source_path}" "${destination_path}"
  if [[ ! -x "${destination_path}" ]] || ! file -b "${destination_path}" | grep -Eq 'Mach-O 64-bit executable x86_64'; then
    printf 'stage-cdp-binaries: staged validation failed: %s\n' "${destination_path}" >&2
    exit 1
  fi
done

printf 'Staged %d Intel macOS CDP binaries in %s\n' "${#executables[@]}" "${destination_dir}"
