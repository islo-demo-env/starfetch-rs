#!/usr/bin/env bash
set -euo pipefail

BINARY="${1:?Usage: corpus.sh <binary> <outdir>}"
OUTDIR="${2:?Usage: corpus.sh <binary> <outdir>}"
DATADIR="${STARFETCH_DATA:-/usr/local/share/starfetch/}"

mkdir -p "$OUTDIR"

# Remove the shuffle-state file so random cases don't pollute deterministic ones
rm -f "$HOME/.fileToParseWithConstellations.txt"

run_case() {
  local name="$1"
  shift
  # Clean shuffle state before each case to ensure determinism
  rm -f "$HOME/.fileToParseWithConstellations.txt"
  STARFETCH_DATA="$DATADIR" "$BINARY" "$@" > "$OUTDIR/${name}.out" 2>&1 || true
  echo $? > "$OUTDIR/${name}.exit"
  # Capture the actual exit code (the || true above makes $? always 0)
  true
}

# Re-implement with proper exit code capture
run_case() {
  local name="$1"
  shift
  rm -f "$HOME/.fileToParseWithConstellations.txt"
  local rc=0
  STARFETCH_DATA="$DATADIR" "$BINARY" "$@" > "$OUTDIR/${name}.out" 2>&1 || rc=$?
  echo "$rc" > "$OUTDIR/${name}.exit"
}

# --- Deterministic cases ---

# help
run_case "help" -h

# list
run_case "list" -l

# orion
run_case "orion" -n orion

# norse-karlvagn
run_case "norse-karlvagn" -n norse karlvagn

# orion-cyan
run_case "orion-cyan" -c cyan -n orion

# missing constellation
run_case "missing" -n missing_name

# --- Structural-only cases (output varies per run) ---

# no-args (random constellation, white)
run_case "no-args"

# -r flag (random constellation)
run_case "random" -r

# -c with no further args (prints available colors)
run_case "colors-only" -c

# -c cyan (random constellation in cyan)
run_case "color-random" -c cyan

echo "Corpus generated in $OUTDIR"
ls -1 "$OUTDIR"
