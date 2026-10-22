#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
BINARY="${REPO_ROOT}/target/debug/starfetch"

if [ ! -x "$BINARY" ]; then
  echo "Error: binary not found at $BINARY — run 'cargo build' first" >&2
  exit 1
fi

export STARFETCH_DATA="${REPO_ROOT}/../res/"
exec /workspace/.parity/corpus.sh "$BINARY" "${1:-/tmp/rust-parity}"
