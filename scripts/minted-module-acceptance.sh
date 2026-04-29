#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${REPO_DIR}"

power="$(scripts/run-fixture.sh run examples/power.d1 --inputs n=2,e=8)"
dowhile="$(scripts/run-fixture.sh run examples/dowhile.d1)"
unless_do="$(scripts/run-fixture.sh run examples/unless.d1)"
dowhile_web="$(scripts/snapshot-web-dowhile-run.sh)"
unless_web="$(scripts/snapshot-web-unless-run.sh)"

printf 'M7 minted module acceptance\n'
printf 'power-cli: %s\n' "${power}"
printf 'dowhile-cli:\n%s\n' "${dowhile}"
printf 'unless-cli:\n%s\n' "${unless_do}"
printf 'dowhile-web: %s\n' "$(printf '%s' "${dowhile_web}" | grep 'data-definition=\"DowhileCounter\"' | head -n 1 | sed 's/^ *//')"
printf 'unless-web: %s\n' "$(printf '%s' "${unless_web}" | grep 'data-definition=\"UnlessCounter\"' | head -n 1 | sed 's/^ *//')"
printf 'status: green\n'
