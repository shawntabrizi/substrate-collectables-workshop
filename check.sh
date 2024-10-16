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
#   --clean                 Run 'cargo clean' after running the checks (optional)
#   --quiet                 Suppress output from Cargo commands (optional)
#
# Examples:
#   ./check.sh --mode check --clean --quiet
#   ./check.sh --mode fix --start 1 --end 5
#
# If no options are provided, the script defaults to 'check' mode, processes all
# directories in 'steps', and skips cargo cleaning.
#
# Note:
# - The script requires 'sccache' to be set as the RUSTC_WRAPPER.
# - Cargo commands are run using the nightly toolchain.

# If there are any errors, stop the script immediately.
set -e

# Use sccache
export RUSTC_WRAPPER=sccache

# Default values
MODE="check"
START=""
END=""
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

# Iterate through each subdirectory in the 'steps' directory in numerical order
for dir in $(ls -d steps/*/ | sort -V); do
  # Extract the directory name
  dir_name=$(basename "$dir")

  # Check if the directory name is a number and within the specified range
  if ! [[ "$dir_name" =~ ^[0-9]+$ ]]; then
    echo "Skipping non-numeric directory: $dir"
    continue
  fi

  if [[ -n "$START" && "$dir_name" -lt "$START" ]]; then
    echo "Skipping directory (below start range): $dir"
    continue
  fi

  if [[ -n "$END" && "$dir_name" -gt "$END" ]]; then
    echo "Skipping directory (above end range): $dir"
    continue
  fi

  # Check if the directory contains a Cargo.toml file
  if [ ! -f "$dir/Cargo.toml" ]; then
    echo "Skipping directory (no Cargo.toml found): $dir"
    continue
  fi

  if [ -d "$dir" ]; then
    echo "Entering directory: $dir"
    cd "$dir"

    # Run cargo fmt and cargo clippy based on the mode
    if [ "$MODE" == "check" ]; then

      echo "Checking cargo fmt"
      RUSTFLAGS="-D warnings" cargo +nightly fmt $QUIET_FLAG -- --check

      echo "Checking cargo clippy"
      RUSTFLAGS="-D warnings" cargo +nightly clippy $QUIET_FLAG

      echo "Checking cargo test"
      RUSTFLAGS="-D warnings" cargo test $QUIET_FLAG

    elif [ "$MODE" == "fix" ]; then

      echo "Running cargo fmt"
      RUSTFLAGS="-D warnings" cargo +nightly fmt $QUIET_FLAG

      echo "Running cargo clippy"
      RUSTFLAGS="-D warnings" cargo +nightly clippy $QUIET_FLAG --fix --allow-dirty

      echo "Running cargo test"
      RUSTFLAGS="-D warnings" cargo test $QUIET_FLAG

    fi

    if [ "$CLEAN" == true ]; then
      echo "Cleaning up cargo"
      cargo clean
    fi

    # Return to the previous directory
    cd - > /dev/null
  fi
done

echo "All operations completed."
