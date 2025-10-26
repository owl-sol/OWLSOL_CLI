#!/bin/bash

clear

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║       🦉  OWLSOL CLI - Complete Demonstration  🦉            ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Generate data
echo "📦 Step 1: Generating Real Solana Account Data..."
./scripts/generate-solana-accounts.sh
echo ""
read -p "Press Enter to continue..."

# Build CLI
echo ""
echo "🔨 Step 2: Building CLI (Release Mode)..."
cargo build --release
echo ""
read -p "Press Enter to continue..."

clear
echo "═══════════════════════════════════════════════════════════════"
echo " TEST 1: Compress NFT Metadata (Auto Algorithm)"
echo "═══════════════════════════════════════════════════════════════"
echo ""
./target/release/owlsol compress -i solana-accounts/metaplex-nft.json
echo ""
read -p "Press Enter to continue..."

clear
echo "═══════════════════════════════════════════════════════════════"
echo " TEST 2: Compress with Huffman Algorithm"
echo "═══════════════════════════════════════════════════════════════"
echo ""
./target/release/owlsol compress -i solana-accounts/spl-token-metadata.json -a huffman
echo ""
read -p "Press Enter to continue..."

clear
echo "═══════════════════════════════════════════════════════════════"
echo " TEST 3: Compress with LZ4 (Fastest)"
echo "═══════════════════════════════════════════════════════════════"
echo ""
./target/release/owlsol compress -i solana-accounts/amm-pool-state.json -a lz4
echo ""
read -p "Press Enter to continue..."

clear
echo "═══════════════════════════════════════════════════════════════"
echo " TEST 4: Compress with Zstd (Best Ratio)"
echo "═══════════════════════════════════════════════════════════════"
echo ""
./target/release/owlsol compress -i solana-accounts/stake-pool-account.json -a zstd
echo ""
read -p "Press Enter to continue..."

clear
echo "═══════════════════════════════════════════════════════════════"
echo " TEST 5: Compress with Dictionary Algorithm"
echo "═══════════════════════════════════════════════════════════════"
echo ""
./target/release/owlsol compress -i solana-accounts/governance-proposal.json -a dictionary
echo ""
read -p "Press Enter to continue..."

clear
echo "═══════════════════════════════════════════════════════════════"
echo " TEST 6: Decompress File"
echo "═══════════════════════════════════════════════════════════════"
echo ""
./target/release/owlsol decompress -i solana-accounts/metaplex-nft.json.owlsol -o decompressed.json
echo ""
echo "Verifying decompressed file..."
diff solana-accounts/metaplex-nft.json decompressed.json && echo "✅ Files match perfectly!" || echo "❌ Files differ!"
echo ""
read -p "Press Enter to continue..."

clear
echo "═══════════════════════════════════════════════════════════════"
echo " TEST 7: Show Statistics"
echo "═══════════════════════════════════════════════════════════════"
echo ""
./target/release/owlsol stats -i solana-accounts/compressed-nft.json --verbose
echo ""
read -p "Press Enter to continue..."

clear
echo "═══════════════════════════════════════════════════════════════"
echo " TEST 8: Benchmark Performance"
echo "═══════════════════════════════════════════════════════════════"
echo ""
./target/release/owlsol benchmark -i solana-accounts/jupiter-route.json -n 100
echo ""
read -p "Press Enter to continue..."

clear
echo "═══════════════════════════════════════════════════════════════"
echo " TEST 9: Batch Processing"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Compressing all JSON files..."
for file in solana-accounts/*.json; do
    echo "  → $(basename \"$file\")"
    ./target/release/owlsol compress -i "$file" 2>/dev/null
done
echo ""
echo "📊 Compression Results:"
echo ""
ls -lh solana-accounts/*.owlsol
echo ""
read -p "Press Enter to continue..."

clear
echo "═══════════════════════════════════════════════════════════════"
echo " TEST 10: Compare All Algorithms"
echo "═══════════════════════════════════════════════════════════════"
echo ""
test_file="solana-accounts/magic-eden-listing.json"
echo "Testing file: $(basename \"$test_file\")"
echo ""

for algo in huffman lz4 zstd dictionary rle; do
    echo "─────────────────────────────────────────────────────────────"
    echo "Algorithm: $algo"
    echo "─────────────────────────────────────────────────────────────"
    ./target/release/owlsol compress -i "$test_file" -a "$algo" -o "/tmp/test-$algo.owlsol" 2>/dev/null
    original_size=$(stat -f%z "$test_file" 2>/dev/null || stat -c%s "$test_file")
    compressed_size=$(stat -f%z "/tmp/test-$algo.owlsol" 2>/dev/null || stat -c%s "/tmp/test-$algo.owlsol")
    ratio=$(echo "scale=2; (1 - $compressed_size / $original_size) * 100" | bc)
    echo "Original: $original_size bytes | Compressed: $compressed_size bytes | Saved: $ratio%"
    echo ""
done
echo ""
read -p "Press Enter to continue..."

clear
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║                   🎉 DEMO COMPLETE! 🎉                       ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "📋 Summary:"
echo ""
echo "✅ Generated 10 real Solana account files"
echo "✅ Built OWLSOL CLI in release mode"
echo "✅ Tested all 5 compression algorithms"
echo "✅ Verified decompression accuracy"
echo "✅ Analyzed statistics and performance"
echo "✅ Benchmarked compression speed"
echo "✅ Compared algorithm efficiency"
echo ""
echo "📁 Files Created:"
ls -1 solana-accounts/*.owlsol 2>/dev/null | head -5
echo "   ... and more"
echo ""
echo "🚀 Next Steps:"
echo "   • Try: ./target/release/owlsol --help"
echo "   • Read: QUICKSTART.md"
echo "   • Explore: solana-accounts/"
echo ""
echo "Thank you for trying OWLSOL CLI! 🦉"
echo ""
