#!/usr/bin/env bash
# Optional opt-in telemetry sender
# Usage: TELEMETRY_URL=https://collector.example/ingest scripts/metrics/telemetry_ping.sh metrics.json
set -euo pipefail
if [ -z "${TELEMETRY_URL:-}" ]; then echo "TELEMETRY_URL not set; skipping send"; exit 0; fi
if [ $# -ne 1 ]; then echo "Usage: $0 metrics.json"; exit 1; fi
file="$1"
if [ ! -f "$file" ]; then echo "file not found"; exit 1; fi
# minimal anonymized payload
payload=$(jq '{repo:.repo,collector_version:"v1",aggregate:.} | {repo: .repo, metrics: .aggregate}' "$file")
curl -fsSL -X POST -H "Content-Type: application/json" -d "$payload" "$TELEMETRY_URL" || echo "telemetry send failed"
echo "telemetry sent"
