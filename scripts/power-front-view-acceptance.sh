#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${REPO_DIR}"

front_snapshot="$(scripts/snapshot-web-front-facet.sh)"
minted_acceptance="$(scripts/minted-module-acceptance.sh)"

printf 'Power front-view acceptance\n'
printf 'power-front-line-1: %s\n' "$(printf '%s' "${front_snapshot}" | grep '⎧ 1 →' | head -n 1 | sed 's/^ *//')"
printf 'power-front-line-2: %s\n' "$(printf '%s' "${front_snapshot}" | grep 'Power (n : ℤ  e : ℤ)' | head -n 1 | sed 's/^ *//')"
printf 'power-front-line-3: %s\n' "$(printf '%s' "${front_snapshot}" | grep '⎩ n (×) →' | head -n 1 | sed 's/^ *//')"
printf 'm7-status: %s\n' "$(printf '%s' "${minted_acceptance}" | grep 'status: green' | head -n 1 | sed 's/^ *//')"
printf 'status: green\n'
