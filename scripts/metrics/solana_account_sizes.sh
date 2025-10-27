#!/usr/bin/env bash
# Query Solana account sizes for a list of addresses
# Usage: scripts/metrics/solana_account_sizes.sh accounts.txt
# Requires: SOLANA_RPC_URL or default mainnet RPC, jq
set -euo pipefail
RPC="${SOLANA_RPC_URL:-https://api.mainnet-beta.solana.com}"
if [ $# -lt 1 ]; then echo "Usage: $0 accounts.txt"; exit 1; fi
INFILE="$1"
echo "[" 
first=true
while read -r addr; do
  [ -z "$addr" ] && continue
  body=$(jq -n --arg acc "$addr" '{jsonrpc:"2.0",id:1,method:"getAccountInfo",params:[$acc,{"encoding":"base64","dataSlice":null}]}' )
  res=$(curl -sSL -X POST -H "Content-Type: application/json" -d "$body" "$RPC")
  data_b64=$(echo "$res" | jq -r '.result.value.data[0] // empty')
  len=0
  if [ -n "$data_b64" ]; then
    len=$(echo "$data_b64" | wc -c)
  fi
  json=$(jq -n --arg addr "$addr" --argjson size "$len" '{account:$addr,bytes_size:$size}')
  if $first; then first=false; else printf ","; fi
  echo "$json"
done < "$INFILE"
echo "]"
