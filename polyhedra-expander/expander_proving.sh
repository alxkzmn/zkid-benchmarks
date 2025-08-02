#!/bin/bash

set -e


# Function to convert bytes to human-readable format
bytes_to_human() {
  local bytes=$1
  local kib=$((1024))
  local mib=$((1024 * kib))
  local gib=$((1024 * mib))

  if (( bytes >= gib )); then
    printf "%.2f GiB" "$(echo "$bytes / $gib" | bc -l)"
  elif (( bytes >= mib )); then
    printf "%.2f MiB" "$(echo "$bytes / $mib" | bc -l)"
  elif (( bytes >= kib )); then
    printf "%.2f KiB" "$(echo "$bytes / $kib" | bc -l)"
  else
    printf "%d B" "$bytes"
  fi
}

# Function to measure memory usage of a command
measure_memory() {
  /usr/bin/time -l "$@" 2>&1 | awk '/maximum resident set size/ {print $1}'
}

RUSTFLAGS="-C target-cpu=native" cargo build --release

measure_memory cargo run --bin=measure --release

# Extract memory usage during proof generation
PROVE_MEM=$(awk '/maximum resident set size/ {print $1}' prove_metrics.txt)
echo "Proof generation memory usage: $(bytes_to_human $PROVE_MEM)"


