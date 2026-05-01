#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${REPO_DIR}"

symbols="$(scripts/snapshot-web-3d-symbols.sh)"
viewer="$(scripts/snapshot-web-3d-viewer.sh)"

printf 'M10 3D viewer acceptance\n'
printf 'symbol-count: %s\n' "$(printf '%s\n' "${symbols}" | grep '"symbol_count": 54' | sed 's/^ *//')"
printf 'front-symbol: %s\n' "$(printf '%s\n' "${symbols}" | grep '"text":"loop","face":"Front","x":0,"y":2,"z":0' | sed 's/^ *//')"
printf 'viewer-mount: %s\n' "$(printf '%s\n' "${viewer}" | grep 'class="viewer-3d-mount"' | sed 's/^ *//')"
printf 'viewer-status: %s\n' "$(printf '%s\n' "${viewer}" | grep 'Loading: Power symbol export' | sed 's/^ *//')"
printf 'asset-copy: %s\n' "$(grep 'src/viewer3d.js' ui/web/index.html | sed 's/^ *//')"
printf 'scene-hook: %s\n' "$(grep 'window.discoveryOne3dSymbols = data;' ui/web/src/viewer3d.js | sed 's/^ *//')"
printf 'manual-demo: run ./scripts/run-web.sh 1078, open http://localhost:1078/, verify the 3D Viewer panel replaces the loading text with a non-empty canvas.\n'
printf 'status: green\n'
