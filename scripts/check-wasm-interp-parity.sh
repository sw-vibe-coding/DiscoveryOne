#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
D1="${REPO_DIR}/target/release/d1"

check_case() {
  local name="$1"
  local file="$2"
  local inputs="$3"
  local interp_output
  local wasm_output

  interp_output="$("${D1}" interp "${file}" --inputs "${inputs}")"
  wasm_output="$("${D1}" run "${file}" --inputs "${inputs}")"

  if [[ "${interp_output}" != "${wasm_output}" ]]; then
    printf 'mismatch %s\n' "${name}" >&2
    printf 'interp: %s\n' "${interp_output}" >&2
    printf 'wasm: %s\n' "${wasm_output}" >&2
    return 1
  fi

  printf 'ok %s => %s\n' "${name}" "${wasm_output}"
}

cd "${REPO_DIR}"

check_case "power_2_8" "examples/power.d1" "n=2,e=8"
check_case "power_5_0" "examples/power.d1" "n=5,e=0"
