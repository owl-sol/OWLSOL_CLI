#!/bin/bash

echo "🧹 Cleaning up OWLSOL test files..."
echo ""

# Remove compressed files
if [ -d "solana-accounts" ]; then
    echo "Removing compressed files..."
    rm -f solana-accounts/*.owlsol
    rm -f solana-accounts/*.owlsol.meta.json
    echo "  ✓ Cleaned solana-accounts/"
fi

# Remove test results
if [ -d "test-results" ]; then
    echo "Removing test results..."
    rm -rf test-results
    echo "  ✓ Cleaned test-results/"
fi

# Remove decompressed test files
if [ -f "decompressed.json" ]; then
    rm -f decompressed.json
    echo "  ✓ Removed decompressed.json"
fi

# Remove benchmark reports
if ls benchmark-report-*.txt 1> /dev/null 2>&1; then
    rm -f benchmark-report-*.txt
    echo "  ✓ Removed benchmark reports"
fi

# Remove temp files
rm -f /tmp/test-*.owlsol 2>/dev/null

echo ""
echo "✅ Cleanup complete!"
echo ""
echo "To start fresh:"
echo "  1. ./scripts/generate-solana-accounts.sh"
echo "  2. cargo build --release"
echo "  3. ./scripts/full-demo.sh"
