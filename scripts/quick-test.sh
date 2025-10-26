#!/bin/bash

echo "⚡ Quick Test - OWLSOL CLI"
echo ""

# Check if binary exists
if [ ! -f "./target/release/owlsol" ]; then
    echo "❌ Binary not found. Building..."
    cargo build --release
fi

# Check if test data exists
if [ ! -d "solana-accounts" ]; then
    echo "❌ Test data not found. Generating..."
    ./scripts/generate-solana-accounts.sh
fi

echo ""
echo "Testing compression with NFT metadata..."
./target/release/owlsol compress -i solana-accounts/metaplex-nft.json

echo ""
echo "Testing decompression..."
./target/release/owlsol decompress -i solana-accounts/metaplex-nft.json.owlsol -o /tmp/test-decompress.json

echo ""
echo "Verifying integrity..."
if diff solana-accounts/metaplex-nft.json /tmp/test-decompress.json > /dev/null 2>&1; then
    echo "✅ Compression/Decompression test PASSED!"
else
    echo "❌ Compression/Decompression test FAILED!"
    exit 1
fi

rm /tmp/test-decompress.json

echo ""
echo "All tests passed! 🎉"
