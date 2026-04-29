#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

cargo run --quiet --manifest-path "${REPO_DIR}/ui/web/Cargo.toml" --example snapshot_dowhile_run
