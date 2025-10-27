#!/usr/bin/env bash
# filepath: scripts/metrics/github_metrics.sh
set -euo pipefail
OWNER="${OWNER:-owl-sol}"
REPO="${REPO:-OWLSOL_CLI}"

if [ -z "${GITHUB_TOKEN:-}" ]; then
  echo '{"error":"GITHUB_TOKEN not set"}' >&2
  echo '{}'
  exit 0
fi

AUTH="Authorization: token ${GITHUB_TOKEN}"

# Fetch repo info with better error handling
repo=$(curl -sSL -H "$AUTH" "https://api.github.com/repos/${OWNER}/${REPO}" 2>/dev/null)
if [ -z "$repo" ] || [ "$repo" = "null" ]; then
  echo '{"error":"Failed to fetch repo info"}' >&2
  echo '{}'
  exit 0
fi

stars=$(echo "$repo" | jq '.stargazers_count // 0' 2>/dev/null || echo 0)
forks=$(echo "$repo" | jq '.forks_count // 0' 2>/dev/null || echo 0)
watchers=$(echo "$repo" | jq '.subscribers_count // 0' 2>/dev/null || echo 0)
open_issues=$(echo "$repo" | jq '.open_issues_count // 0' 2>/dev/null || echo 0)

# Fetch releases
releases=$(curl -sSL -H "$AUTH" "https://api.github.com/repos/${OWNER}/${REPO}/releases" 2>/dev/null || echo '[]')
download_count=$(echo "$releases" | jq 'if type == "array" then [.[] | .assets[]? | .download_count] | add // 0 else 0 end' 2>/dev/null || echo 0)
release_count=$(echo "$releases" | jq 'if type == "array" then length else 0 end' 2>/dev/null || echo 0)

# Fetch traffic (requires push access)
views=$(curl -sSL -H "$AUTH" "https://api.github.com/repos/${OWNER}/${REPO}/traffic/views" 2>/dev/null || echo '{"count":0,"uniques":0}')
clones=$(curl -sSL -H "$AUTH" "https://api.github.com/repos/${OWNER}/${REPO}/traffic/clones" 2>/dev/null || echo '{"count":0,"uniques":0}')

# Get contributors count
contributors=$(curl -sSL -H "$AUTH" "https://api.github.com/repos/${OWNER}/${REPO}/contributors?per_page=1" 2>/dev/null)
contributor_count=$(echo "$contributors" | jq 'if type == "array" then length else 0 end' 2>/dev/null || echo 0)

jq -n --arg owner "$OWNER" --arg repo "$REPO" \
  --argjson stars "$stars" --argjson forks "$forks" --argjson watchers "$watchers" \
  --argjson issues "$open_issues" --argjson downloads "$download_count" \
  --argjson releases "$release_count" --argjson contributors "$contributor_count" \
  --argjson views "$views" --argjson clones "$clones" \
  '{
    owner:$owner,
    repo:$repo,
    stars:$stars,
    forks:$forks,
    watchers:$watchers,
    open_issues:$issues,
    release_count:$releases,
    release_downloads:$downloads,
    contributors:$contributors,
    views:$views,
    clones:$clones
  }'