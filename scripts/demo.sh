#!/bin/bash

echo "🦉 OWLSOL CLI Demo"
echo "=================="
echo ""

# Generate test data first
./scripts/generate-test-data.sh

echo ""
echo "📦 1. COMPRESS NFT METADATA"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo run --release --bin owlsol -- compress -i test-data/nft-metadata.json

echo ""
echo "📦 2. COMPRESS GAME PROFILE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo run --release --bin owlsol -- compress -i test-data/game-profile.json -a lz4

echo ""
echo "📦 3. COMPRESS WITH SPECIFIC ALGORITHM"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo run --release --bin owlsol -- compress -i test-data/defi-state.json -a zstd

echo ""
echo "📊 4. SHOW STATISTICS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo run --release --bin owlsol -- stats -i test-data/nft-metadata.json --verbose

echo ""
echo "🔓 5. DECOMPRESS FILE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo run --release --bin owlsol -- decompress -i test-data/nft-metadata.json.owlsol -o test-data/nft-decompressed.json

echo ""
echo "⚡ 6. BENCHMARK"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cargo run --release --bin owlsol -- benchmark -i test-data/game-profile.json -n 1000

echo ""
echo "✅ Demo Complete!"
