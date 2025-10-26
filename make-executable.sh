#!/bin/bash

echo "Making all scripts executable..."

chmod +x scripts/generate-solana-accounts.sh
chmod +x scripts/full-demo.sh
chmod +x scripts/test-all-algorithms.sh
chmod +x scripts/benchmark-all.sh
chmod +x scripts/cleanup.sh
chmod +x scripts/quick-test.sh
chmod +x SETUP.sh

echo "✅ All scripts are now executable!"
echo ""
echo "Run './SETUP.sh' to begin setup."
