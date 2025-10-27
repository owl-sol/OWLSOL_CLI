#!/usr/bin/env bash
# filepath: scripts/metrics/collect_metrics.sh
set -euo pipefail

OUTDIR="${OUTDIR:-metrics_out}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
mkdir -p "$OUTDIR"

echo "🦉 OWLSOL Metrics Collection"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Output directory: $OUTDIR"
echo "Timestamp: $TIMESTAMP"
echo ""

# GitHub metrics
echo "📊 Collecting GitHub metrics..."
if ./scripts/metrics/github_metrics.sh > "$OUTDIR/github.json" 2>/dev/null; then
  echo "   ✓ GitHub metrics collected"
else
  echo "   ⚠ GitHub metrics failed (check GITHUB_TOKEN)"
  echo "{}" > "$OUTDIR/github.json"
fi

# CI metrics
echo "🔄 Collecting CI metrics..."
if ./scripts/metrics/ci_metrics.sh > "$OUTDIR/ci.json" 2>/dev/null; then
  echo "   ✓ CI metrics collected"
else
  echo "   ⚠ CI metrics failed"
  echo "{}" > "$OUTDIR/ci.json"
fi

# Compression benchmarks (if samples exist)
if [ -d "metrics_samples" ] && [ "$(ls -A metrics_samples 2>/dev/null)" ]; then
  echo "⚡ Running compression benchmarks..."
  bench_count=0
  for sample in metrics_samples/*; do
    if [ -f "$sample" ] && command -v owlsol >/dev/null 2>&1; then
      sample_name=$(basename "$sample")
      if ./scripts/metrics/compression_bench.sh "$sample" > "$OUTDIR/bench_${sample_name}.json" 2>/dev/null; then
        echo "   ✓ Benchmarked: $sample_name"
        bench_count=$((bench_count + 1))
      fi
    fi
  done
  echo "   Completed $bench_count benchmark(s)"
fi

# Combine all metrics
echo ""
echo "📦 Aggregating results..."
jq -s 'reduce .[] as $item ({}; . * $item)' "$OUTDIR"/*.json 2>/dev/null > "$OUTDIR/aggregate_${TIMESTAMP}.json" || echo "{}" > "$OUTDIR/aggregate_${TIMESTAMP}.json"

# Create symlink to latest
ln -sf "aggregate_${TIMESTAMP}.json" "$OUTDIR/aggregate.json"

# Optional telemetry
if [ -n "${TELEMETRY_URL:-}" ] && [ -f "$OUTDIR/aggregate.json" ]; then
  echo "📡 Sending telemetry..."
  if TELEMETRY_URL="${TELEMETRY_URL}" ./scripts/metrics/telemetry_ping.sh "$OUTDIR/aggregate.json" 2>/dev/null; then
    echo "   ✓ Telemetry sent"
  else
    echo "   ⚠ Telemetry send failed"
  fi
fi

echo ""
echo "✅ Metrics collection complete!"
echo "   Latest: $OUTDIR/aggregate.json"
echo "   Timestamped: $OUTDIR/aggregate_${TIMESTAMP}.json"
echo ""
echo "View results: cat $OUTDIR/aggregate.json | jq ."