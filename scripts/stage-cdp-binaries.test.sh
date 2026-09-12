#!/usr/bin/env bash
set -euo pipefail

# Self-contained regression tests for stage-cdp-binaries.sh. Run with:
#   bash scripts/stage-cdp-binaries.test.sh
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
script_source="${repo_root}/scripts/stage-cdp-binaries.sh"
executables=(modify sfedit pvoc isolate sfprops blur bounce combine envel envnu extend filter flatten focus hover hover2 iterline iterlinef phasor reverb sfecho spectstr stretch submix)
test_root="$(mktemp -d "${TMPDIR:-/tmp}/stage-cdp-binaries-test.XXXXXX")"
trap 'rm -rf "${test_root}"' EXIT

fail() { printf 'FAIL: %s\n' "$*" >&2; exit 1; }

make_fixture_repo() {
  local fixture=$1
  mkdir -p "${fixture}/scripts" "${fixture}/src-tauri/binaries" "${fixture}/mock-bin"
  cp "${script_source}" "${fixture}/scripts/stage-cdp-binaries.sh"
  chmod +x "${fixture}/scripts/stage-cdp-binaries.sh"
  printf '#!/usr/bin/env bash\nprintf "%%s\\n" "${FILE_DESCRIPTION:?}"\n' >"${fixture}/mock-bin/file"
  chmod +x "${fixture}/mock-bin/file"
}

make_sources() {
  local source_dir=$1 extension=$2
  mkdir -p "${source_dir}"
  for executable in "${executables[@]}"; do
    printf 'fixture %s\n' "${executable}" >"${source_dir}/${executable}${extension}"
    chmod +x "${source_dir}/${executable}${extension}"
  done
}

run_matrix_case() {
  local triple=$1 extension=$2 description=$3
  local fixture="${test_root}/${triple}" source_dir="${test_root}/${triple}/source"
  make_fixture_repo "${fixture}"
  make_sources "${source_dir}" "${extension}"
  PATH="${fixture}/mock-bin:${PATH}" FILE_DESCRIPTION="${description}" \
    "${fixture}/scripts/stage-cdp-binaries.sh" "${source_dir}" "${triple}" >/dev/null
  for executable in "${executables[@]}"; do
    [[ -f "${fixture}/src-tauri/binaries/${executable}-${triple}${extension}" ]] || fail "${triple} did not stage ${executable}"
  done
  if [[ -z "${extension}" ]]; then
    [[ -x "${fixture}/src-tauri/binaries/modify-${triple}" ]] || fail "${triple} did not preserve executable mode"
  fi
}

run_matrix_case x86_64-apple-darwin '' 'Mach-O 64-bit executable x86_64'
run_matrix_case aarch64-apple-darwin '' 'Mach-O 64-bit executable arm64'
run_matrix_case x86_64-pc-windows-msvc .exe 'PE32+ executable (console) x86-64, for MS Windows'
run_matrix_case aarch64-pc-windows-msvc .exe 'PE32+ executable (console) Aarch64, for MS Windows'
run_matrix_case x86_64-unknown-linux-gnu '' 'ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV)'
run_matrix_case aarch64-unknown-linux-gnu '' 'ELF 64-bit LSB pie executable, ARM aarch64, version 1 (SYSV)'

unknown_fixture="${test_root}/unknown"
make_fixture_repo "${unknown_fixture}"
if PATH="${unknown_fixture}/mock-bin:${PATH}" FILE_DESCRIPTION='unused' \
  "${unknown_fixture}/scripts/stage-cdp-binaries.sh" /does/not/matter wasm32-wasi >/dev/null 2>&1; then
  fail 'unknown target was accepted'
fi

# A bad member must leave an existing final binary untouched, proving that the
# source validation pass happens before the script starts replacing outputs.
failure_fixture="${test_root}/failure"
failure_source="${failure_fixture}/source"
make_fixture_repo "${failure_fixture}"
make_sources "${failure_source}" ''
rm "${failure_source}/sfprops"
sentinel="${failure_fixture}/src-tauri/binaries/modify-x86_64-apple-darwin"
printf 'original destination\n' >"${sentinel}"
if PATH="${failure_fixture}/mock-bin:${PATH}" FILE_DESCRIPTION='Mach-O 64-bit executable x86_64' \
  "${failure_fixture}/scripts/stage-cdp-binaries.sh" "${failure_source}" x86_64-apple-darwin >/dev/null 2>&1; then
  fail 'incomplete source set was accepted'
fi
[[ "$(<"${sentinel}")" == 'original destination' ]] || fail 'failed staging changed a final destination'

# Matching source names are insufficient: a foreign object format must also be
# rejected before any final output is replaced.
wrong_arch_fixture="${test_root}/wrong-architecture"
wrong_arch_source="${wrong_arch_fixture}/source"
make_fixture_repo "${wrong_arch_fixture}"
make_sources "${wrong_arch_source}" ''
sentinel="${wrong_arch_fixture}/src-tauri/binaries/modify-x86_64-apple-darwin"
printf 'original destination\n' >"${sentinel}"
if PATH="${wrong_arch_fixture}/mock-bin:${PATH}" FILE_DESCRIPTION='ELF 64-bit LSB pie executable, x86-64' \
  "${wrong_arch_fixture}/scripts/stage-cdp-binaries.sh" "${wrong_arch_source}" x86_64-apple-darwin >/dev/null 2>&1; then
  fail 'foreign object format was accepted'
fi
[[ "$(<"${sentinel}")" == 'original destination' ]] || fail 'foreign architecture changed a final destination'

printf 'stage-cdp-binaries tests passed\n'
