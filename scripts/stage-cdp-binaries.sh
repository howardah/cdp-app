#!/usr/bin/env bash
set -euo pipefail

# Copy the approved CDP sidecars from a *known target build* into Tauri's
# target-suffixed sidecar directory. The target is deliberately an argument:
# a filename in a release tree is not evidence of its architecture.
usage() {
  printf 'Usage: %s <source-directory> <target-triple>\n' "${0##*/}" >&2
}

if [[ $# -ne 2 ]]; then
  usage
  exit 2
fi

source_dir=$1
target_triple=$2
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
destination_dir="${repo_root}/src-tauri/binaries"

case "${target_triple}" in
  x86_64-apple-darwin)
    source_extension='' destination_extension='' object_description='x86_64 Mach-O executable'
    file_pattern='Mach-O.*(x86_64|x86-64)' require_executable_bit=true ;;
  aarch64-apple-darwin)
    source_extension='' destination_extension='' object_description='ARM64 Mach-O executable'
    file_pattern='Mach-O.*(arm64|aarch64)' require_executable_bit=true ;;
  x86_64-pc-windows-msvc)
    source_extension='.exe' destination_extension='.exe' object_description='x86_64 Windows PE executable'
    file_pattern='PE32\+.*(x86-64|x86_64)' require_executable_bit=false ;;
  aarch64-pc-windows-msvc)
    source_extension='.exe' destination_extension='.exe' object_description='ARM64 Windows PE executable'
    file_pattern='PE32\+.*(Aarch64|ARM aarch64|aarch64)' require_executable_bit=false ;;
  x86_64-unknown-linux-gnu)
    source_extension='' destination_extension='' object_description='x86_64 ELF executable'
    file_pattern='ELF 64-bit.*(x86-64|x86_64)' require_executable_bit=true ;;
  aarch64-unknown-linux-gnu)
    source_extension='' destination_extension='' object_description='ARM64 ELF executable'
    file_pattern='ELF 64-bit.*(ARM aarch64|aarch64)' require_executable_bit=true ;;
  *)
    printf 'stage-cdp-binaries: unsupported target triple: %s\n' "${target_triple}" >&2
    exit 2 ;;
esac

command -v file >/dev/null 2>&1 || {
  printf '%s\n' "stage-cdp-binaries: the 'file' command is required" >&2
  exit 1
}

executables=(modify sfedit pvoc isolate sfprops blur bounce combine envel envnu extend filter flatten focus hover hover2 iterline iterlinef phasor reverb sfecho spectstr stretch submix)

validate_binary() {
  local path=$1 phase=$2
  if [[ ! -f "${path}" ]]; then
    printf 'stage-cdp-binaries: missing %s: %s\n' "${phase}" "${path}" >&2
    return 1
  fi
  if [[ "${require_executable_bit}" == true && ! -x "${path}" ]]; then
    printf 'stage-cdp-binaries: %s is not executable: %s\n' "${phase}" "${path}" >&2
    return 1
  fi
  if ! file -b "${path}" | grep -Eqi "${file_pattern}"; then
    printf 'stage-cdp-binaries: %s is not a %s: %s\n' "${phase}" "${object_description}" "${path}" >&2
    return 1
  fi
}

# Validate the complete input set before creating any replacement in the final
# destination. This makes a missing or foreign-architecture sidecar harmless.
for executable in "${executables[@]}"; do
  validate_binary "${source_dir}/${executable}${source_extension}" 'source'
done

mkdir -p "${destination_dir}"
staging_dir="$(mktemp -d "${destination_dir}/.stage-cdp-binaries.XXXXXX")"
cleanup() { rm -rf "${staging_dir}"; }
trap cleanup EXIT

# Prepare and validate every output in an isolated directory first. cp -p
# preserves Unix mode bits; Windows executable mode is not meaningful.
for executable in "${executables[@]}"; do
  source_path="${source_dir}/${executable}${source_extension}"
  staged_path="${staging_dir}/${executable}-${target_triple}${destination_extension}"
  cp -p "${source_path}" "${staged_path}"
  validate_binary "${staged_path}" 'staged binary'
done

# rename(2), used by mv within this directory, atomically replaces each target
# sidecar. All validation above has completed before any final path is changed.
for executable in "${executables[@]}"; do
  mv -f "${staging_dir}/${executable}-${target_triple}${destination_extension}" \
    "${destination_dir}/${executable}-${target_triple}${destination_extension}"
done

printf 'Staged %d CDP binaries for %s in %s\n' \
  "${#executables[@]}" "${target_triple}" "${destination_dir}"
