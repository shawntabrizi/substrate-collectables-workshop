#!/bin/bash

# If there are any errors, stop the script immediately.
set -e

# Use sccache
export RUSTC_WRAPPER=sccache

# Default to 'check' mode if no argument is provided
MODE=${1:-check}
START=${2:-}
END=${3:-}

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
      cargo +nightly fmt -- --check

      echo "Checking cargo clippy"
      RUSTFLAGS="-A unused" cargo +nightly clippy -- -D warnings

      echo "Checking cargo test"
      RUSTFLAGS="-A unused -D warnings" cargo test

    elif [ "$MODE" == "fix" ]; then

      echo "Running cargo fmt"
      RUSTFLAGS="-A unused" cargo +nightly fmt

      echo "Running cargo clippy"
      RUSTFLAGS="-A unused" cargo +nightly clippy --fix --allow-dirty

      echo "Running cargo test"
      RUSTFLAGS="-A unused -D warnings" cargo test

    fi

    # Return to the previous directory
    cd - > /dev/null
  fi
done

echo "All operations completed."
