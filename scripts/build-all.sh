#!/usr/bin/env bash
#
# Build the DiscoveryOne CLI and web shell.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
WEB_DIR="${REPO_DIR}/ui/web"

if ! command -v trunk >/dev/null 2>&1; then
  echo "build-all.sh: trunk is required; install with: cargo install trunk" >&2
  exit 1
fi

cargo build --release -p d1-cli

mkdir -p "${REPO_DIR}/target"
LOCK="${REPO_DIR}/target/.trunk-dist.lock"
if ! mkdir "${LOCK}" 2>/dev/null; then
  HOLDER="$(cat "${LOCK}/pid" 2>/dev/null || echo unknown)"
  if [ "${HOLDER}" != "unknown" ] && ! kill -0 "${HOLDER}" 2>/dev/null; then
    echo "build-all.sh: removing stale lock from pid ${HOLDER}" >&2
    rm -rf "${LOCK}"
    mkdir "${LOCK}"
  else
    echo "build-all.sh: another trunk process (pid ${HOLDER}) holds ${LOCK} -- refusing to share dist/" >&2
    exit 1
  fi
fi
echo $$ > "${LOCK}/pid"
trap 'rm -rf "${LOCK}"' EXIT INT TERM

cd "${WEB_DIR}"
env -u NO_COLOR TRUNK_COLOR=never trunk build --release
