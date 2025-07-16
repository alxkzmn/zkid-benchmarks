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

# Function to record stats of a command
time_fn() {
  /usr/bin/time -l -o cargo_time.log "$@"
}

# Run the SHA256 hash proving & record the stats
time_fn cargo run --release -- big-sha2

# Get the memory size needed for whole process
max_rss_bytes=$(awk '/maximum resident set size/ {print $1}' cargo_time.log)

echo "Proving memory usage: $(bytes_to_human $max_rss_bytes)"
echo "NOTE: Table data is saved in 'metrics.csv' file. There, unit of duration is nanoseconds."
