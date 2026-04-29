#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${REPO_DIR}"

definition_order() {
    grep -o 'data-definition="[^"]*"' | sed 's/data-definition="//;s/"//'
}

inline_order() {
    paste -sd ' ' -
}

default_grid="$(scripts/snapshot-web-library-grid.sh)"
name_grid="$(scripts/snapshot-web-library-grid-name.sh)"
arity_grid="$(scripts/snapshot-web-library-grid-arity.sh)"
type_grid="$(scripts/snapshot-web-library-grid-type.sh)"
aspects_grid="$(scripts/snapshot-web-library-grid-aspects.sh)"

printf 'M8 library grid acceptance\n'
printf 'default-count: %s\n' "$(printf '%s\n' "${default_grid}" | definition_order | wc -l | tr -d ' ')"
printf 'default-order: %s\n' "$(printf '%s\n' "${default_grid}" | definition_order | inline_order)"
printf 'sort-name: %s\n' "$(printf '%s\n' "${name_grid}" | definition_order | inline_order)"
printf 'sort-arity: %s\n' "$(printf '%s\n' "${arity_grid}" | definition_order | inline_order)"
printf 'sort-type: %s\n' "$(printf '%s\n' "${type_grid}" | definition_order | inline_order)"
printf 'sort-aspects: %s\n' "$(printf '%s\n' "${aspects_grid}" | definition_order | inline_order)"
printf 'status: green\n'
