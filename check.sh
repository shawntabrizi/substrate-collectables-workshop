#!/bin/bash

# This script runs various Cargo commands (fmt, clippy, test) in directories
# containing a Cargo.toml file inside the 'steps' directory. It allows checking
# or fixing code formatting and linting issues with optional cargo cleaning.
#
# Usage:
#   ./check.sh [OPTIONS]
#
# Options:
#   --mode [check|fix]      Specify the mode of operation (default: check)
#                           - 'check': Run cargo fmt, clippy, and test in check mode
#                           - 'fix': Run cargo fmt and clippy in fix mode
#
#   --start [number]        Specify the start directory (inclusive) based on numerical order (optional)
#   --end [number]          Specify the end directory (inclusive) based on numerical order (optional)
#
#   --jobs [number]         Max parallel jobs (default: number of CPU cores)
#   --clean                 Run 'cargo clean' after running the checks (optional)
#   --quiet                 Suppress output from Cargo commands (optional)
#
# Examples:
#   ./check.sh --mode check --clean --quiet
#   ./check.sh --mode fix --start 1 --end 5
#   ./check.sh --mode check --jobs 4
#
# If no options are provided, the script defaults to 'check' mode, processes all
# directories in 'steps', and skips cargo cleaning.
#
# Note:
# - Cargo commands are run using the nightly toolchain.

# Enable sccache if available
if command -v sccache &>/dev/null; then
  export RUSTC_WRAPPER=sccache
  echo "Using sccache for compilation caching"
else
  echo "Warning: sccache not found, running without compilation cache"
fi

# Default values
MODE="check"
START=""
END=""
MAX_JOBS=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)
CLEAN=false
QUIET=false

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --mode)
      MODE="$2"
      shift 2
      ;;
    --start)
      START="$2"
      shift 2
      ;;
    --end)
      END="$2"
      shift 2
      ;;
    --jobs)
      MAX_JOBS="$2"
      shift 2
      ;;
    --clean)
      CLEAN=true
      shift 1
      ;;
    --quiet)
      QUIET=true
      shift 1
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Check if the mode is valid
if [[ "$MODE" != "check" && "$MODE" != "fix" ]]; then
  echo "Invalid mode: $MODE. Use 'check' or 'fix'."
  exit 1
fi

# Check if the 'steps' directory exists
if [ ! -d "steps" ]; then
  echo "Directory 'steps' does not exist."
  exit 1
fi

# Add --quiet flag to Cargo commands if QUIET is true
QUIET_FLAG=""
if [ "$QUIET" == true ]; then
  QUIET_FLAG="--quiet"
fi

ROOT_DIR=$(pwd)
FAILED_FILE=$(mktemp)
echo -n "" > "$FAILED_FILE"

# Process a single step directory
check_dir() {
  local dir="$1"
  local dir_name="$2"
  local mode="$3"
  local quiet_flag="$4"
  local clean="$5"
  local label="$dir"

  if [ "$mode" == "check" ]; then

    echo "[$label] Checking cargo fmt"
    if ! (cd "$dir" && cargo +nightly fmt $quiet_flag -- --check) 2>&1; then
      echo "FAIL: $label (fmt)" >> "$FAILED_FILE"
      return 1
    fi

    echo "[$label] Checking cargo clippy"
    if ! (cd "$dir" && RUSTFLAGS="-A unused" cargo +nightly clippy $quiet_flag -- -D warnings) 2>&1; then
      echo "FAIL: $label (clippy)" >> "$FAILED_FILE"
      return 1
    fi

    echo "[$label] Checking cargo test"
    if ! (cd "$dir" && RUSTFLAGS="-A unused -D warnings" cargo test $quiet_flag) 2>&1; then
      echo "FAIL: $label (test)" >> "$FAILED_FILE"
      return 1
    fi

  elif [ "$mode" == "fix" ]; then

    echo "[$label] Running cargo fmt"
    if ! (cd "$dir" && cargo +nightly fmt $quiet_flag) 2>&1; then
      echo "FAIL: $label (fmt)" >> "$FAILED_FILE"
      return 1
    fi

    echo "[$label] Running cargo clippy"
    if ! (cd "$dir" && RUSTFLAGS="-A unused" cargo +nightly clippy $quiet_flag --fix --allow-dirty) 2>&1; then
      echo "FAIL: $label (clippy)" >> "$FAILED_FILE"
      return 1
    fi

    echo "[$label] Running cargo test"
    if ! (cd "$dir" && RUSTFLAGS="-A unused -D warnings" cargo test $quiet_flag) 2>&1; then
      echo "FAIL: $label (test)" >> "$FAILED_FILE"
      return 1
    fi

  fi

  if [ "$clean" == true ]; then
    echo "[$label] Cleaning up cargo"
    (cd "$dir" && cargo clean)
  fi

  echo "[$label] OK"
}

export -f check_dir
export FAILED_FILE
export RUSTC_WRAPPER

echo "Running in '$MODE' mode with up to $MAX_JOBS parallel jobs"
echo "---"

# Collect all directories to process
DIRS=()
NAMES=()
for dir in $(ls -d steps/*/ | sort -V); do
  dir_name=$(basename "$dir")

  # Skip non-numeric directories
  if ! [[ "$dir_name" =~ ^[0-9]+$ ]]; then
    continue
  fi

  if [[ -n "$START" && "$dir_name" -lt "$START" ]]; then
    continue
  fi

  if [[ -n "$END" && "$dir_name" -gt "$END" ]]; then
    continue
  fi

  if [ ! -f "$dir/Cargo.toml" ]; then
    echo "Skipping directory (no Cargo.toml found): $dir"
    continue
  fi

  DIRS+=("$ROOT_DIR/$dir")
  NAMES+=("$dir_name")
done

echo "Found ${#DIRS[@]} directories to process"
echo "---"

# Run jobs in parallel with a job slot limiter
PIDS=()
JOB_COUNT=0

for i in "${!DIRS[@]}"; do
  # Wait for a slot if we've hit the max
  while (( JOB_COUNT >= MAX_JOBS )); do
    wait -n 2>/dev/null || true
    JOB_COUNT=$(jobs -rp | wc -l | tr -d ' ')
  done

  check_dir "${DIRS[$i]}" "${NAMES[$i]}" "$MODE" "$QUIET_FLAG" "$CLEAN" &
  PIDS+=($!)
  JOB_COUNT=$(jobs -rp | wc -l | tr -d ' ')
done

# Wait for all remaining jobs
EXIT_CODE=0
for pid in "${PIDS[@]}"; do
  wait "$pid" || EXIT_CODE=1
done

echo "---"

# Report results
if [ -s "$FAILED_FILE" ]; then
  echo "FAILURES:"
  cat "$FAILED_FILE"
  rm -f "$FAILED_FILE"
  exit 1
else
  rm -f "$FAILED_FILE"
  echo "All operations completed successfully."
  exit 0
fi
