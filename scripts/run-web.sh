#!/usr/bin/env bash
#
# Start the DiscoveryOne web dev server.
set -euo pipefail

PORT="${1:-9735}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
WEB_DIR="${REPO_DIR}/ui/web"

if ! command -v trunk >/dev/null 2>&1; then
  echo "run-web.sh: trunk is required; install with: cargo install trunk" >&2
  exit 1
fi

mkdir -p "${REPO_DIR}/target"
# Lock lives outside dist/ because Trunk may wipe dist/ on rebuild.
LOCK="${REPO_DIR}/target/.trunk-dist.lock"
if ! mkdir "${LOCK}" 2>/dev/null; then
  HOLDER="$(cat "${LOCK}/pid" 2>/dev/null || echo unknown)"
  if [ "${HOLDER}" != "unknown" ] && ! kill -0 "${HOLDER}" 2>/dev/null; then
    echo "run-web.sh: removing stale lock from pid ${HOLDER}" >&2
    rm -rf "${LOCK}"
    mkdir "${LOCK}"
  else
    echo "run-web.sh: another trunk process (pid ${HOLDER}) holds ${LOCK} -- refusing to share dist/" >&2
    exit 1
  fi
fi
echo $$ > "${LOCK}/pid"
trap 'rm -rf "${LOCK}"' EXIT INT TERM

cd "${WEB_DIR}"
env -u NO_COLOR TRUNK_COLOR=never trunk serve --address 127.0.0.1 --port "${PORT}"
