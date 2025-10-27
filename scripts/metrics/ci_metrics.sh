#!/usr/bin/env bash
# filepath: scripts/metrics/ci_metrics.sh
set -euo pipefail
OWNER="${OWNER:-owl-sol}"
REPO="${REPO:-OWLSOL_CLI}"
WORKFLOW_FILE="${WORKFLOW_FILE:-.github/workflows/nightly.yml}"

if [ -z "${GITHUB_TOKEN:-}" ]; then
  echo '{"error":"GITHUB_TOKEN not set"}' >&2
  echo '{}'
  exit 0
fi

AUTH="Authorization: token ${GITHUB_TOKEN}"

# Get workflow ID
workflows=$(curl -sSL -H "$AUTH" "https://api.github.com/repos/${OWNER}/${REPO}/actions/workflows" 2>/dev/null)
if [ -z "$workflows" ] || [ "$workflows" = "null" ]; then
  echo '{"error":"Failed to fetch workflows"}' >&2
  echo '{}'
  exit 0
fi

wf=$(echo "$workflows" | jq -r --arg wf "$WORKFLOW_FILE" '.workflows[]? | select(.path==$wf) | .id' 2>/dev/null || echo "")

if [ -z "$wf" ] || [ "$wf" = "null" ]; then
  echo '{"error":"Workflow not found","workflow":"'$WORKFLOW_FILE'","total_runs":0}' >&2
  echo '{"workflow":"'$WORKFLOW_FILE'","total_runs":0}'
  exit 0
fi

# Get workflow runs
runs=$(curl -sSL -H "$AUTH" "https://api.github.com/repos/${OWNER}/${REPO}/actions/workflows/${wf}/runs?per_page=100" 2>/dev/null)
if [ -z "$runs" ] || [ "$runs" = "null" ]; then
  echo '{"error":"Failed to fetch runs"}' >&2
  echo '{}'
  exit 0
fi

total=$(echo "$runs" | jq '.total_count // 0' 2>/dev/null || echo 0)

if [ "$total" -eq 0 ]; then 
  echo '{"workflow":"'$WORKFLOW_FILE'","total_runs":0,"success":0,"fail":0,"success_rate":0}'
  exit 0
fi

success_count=$(echo "$runs" | jq '[.workflow_runs[]? | select(.conclusion=="success")] | length' 2>/dev/null || echo 0)
fail_count=$(echo "$runs" | jq '[.workflow_runs[]? | select(.conclusion=="failure" or .conclusion=="cancelled" or .conclusion=="timed_out")] | length' 2>/dev/null || echo 0)

# Calculate success rate
success_rate=$(awk -v s="$success_count" -v t="$total" 'BEGIN{if(t>0) printf "%.2f", (s/t)*100; else print 0}')

# Get average duration (in seconds, not ms)
avg_duration_sec=$(echo "$runs" | jq '[.workflow_runs[]? | (.updated_at | fromdateiso8601) - (.created_at | fromdateiso8601)] | if length > 0 then (add / length) else 0 end' 2>/dev/null || echo 0)

# Get last run status
last_run_status=$(echo "$runs" | jq -r '.workflow_runs[0]?.conclusion // "unknown"' 2>/dev/null || echo "unknown")
last_run_time=$(echo "$runs" | jq -r '.workflow_runs[0]?.created_at // "unknown"' 2>/dev/null || echo "unknown")

jq -n --arg wf "$WORKFLOW_FILE" \
  --argjson total "$total" \
  --argjson success "$success_count" \
  --argjson fail "$fail_count" \
  --argjson rate "$success_rate" \
  --argjson avg_sec "$avg_duration_sec" \
  --arg last_status "$last_run_status" \
  --arg last_time "$last_run_time" \
  '{
    workflow:$wf,
    total_runs:$total,
    success:$success,
    fail:$fail,
    success_rate:$rate,
    avg_duration_seconds:$avg_sec,
    last_run_status:$last_status,
    last_run_time:$last_time
  }'