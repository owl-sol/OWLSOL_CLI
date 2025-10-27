#!/usr/bin/env bash
# filepath: scripts/metrics/compression_bench.sh
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Usage: $0 inputfile" >&2
  exit 1
fi

in="$1"
if [ ! -f "$in" ]; then
  echo "Error: Input file not found: $in" >&2
  exit 1
fi

if ! command -v owlsol >/dev/null 2>&1; then
  echo "Error: owlsol not found in PATH" >&2
  exit 1
fi

name=$(basename "$in")
# Cross-platform stat command
if stat -c%s "$in" >/dev/null 2>&1; then
  orig_size=$(stat -c%s "$in")
else
  orig_size=$(stat -f%z "$in")
fi

out="${in}.owlsol"
meta="${out}.meta.json"

# Remove old files if they exist
rm -f "$out" "$meta"

# Measure compression time
if date +%s%3N >/dev/null 2>&1; then
  start=$(date +%s%3N)
  owlsol compress "$in" -o "$out" 2>&1 | grep -v "^$" || true
  end=$(date +%s%3N)
  duration_ms=$((end-start))
else
  # Fallback for systems without millisecond precision
  start=$(date +%s)
  owlsol compress "$in" -o "$out" 2>&1 | grep -v "^$" || true
  end=$(date +%s)
  duration_ms=$(((end-start)*1000))
fi

if [ ! -f "$out" ]; then
  echo "Error: Compression failed, output file not created" >&2
  exit 1
fi

# Get compressed size
if stat -c%s "$out" >/dev/null 2>&1; then
  compressed_size=$(stat -c%s "$out")
else
  compressed_size=$(stat -f%z "$out")
fi

# Calculate metrics
ratio=$(awk -v a="$compressed_size" -v b="$orig_size" 'BEGIN{printf "%.4f", a/b}')
savings=$(awk -v a="$compressed_size" -v b="$orig_size" 'BEGIN{printf "%.2f", ((b-a)/b)*100}')

# Extract algorithm from metadata if available
algorithm="unknown"
if [ -f "$meta" ]; then
  algorithm=$(jq -r '.algorithm // "unknown"' "$meta" 2>/dev/null || echo "unknown")
fi

jq -n --arg file "$name" \
  --argjson orig "$orig_size" \
  --argjson comp "$compressed_size" \
  --argjson dur "$duration_ms" \
  --argjson ratio "$ratio" \
  --argjson savings "$savings" \
  --arg algo "$algorithm" \
  '{
    file:$file,
    original_bytes:$orig,
    compressed_bytes:$comp,
    compression_ratio:$ratio,
    space_savings_percent:$savings,
    algorithm:$algo,
    duration_ms:$dur
  }'

# Cleanup
rm -f "$out" "$meta"