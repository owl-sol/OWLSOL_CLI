#!/bin/bash

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║          Algorithm Comparison Test Suite                      ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

if [ ! -d "solana-accounts" ]; then
    echo "❌ Error: solana-accounts directory not found"
    echo "Run: ./scripts/generate-solana-accounts.sh"
    exit 1
fi

# Create results directory
mkdir -p test-results

# Test each file with all algorithms
for input_file in solana-accounts/*.json; do
    filename=$(basename "$input_file")
    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "Testing: $filename"
    echo "═══════════════════════════════════════════════════════════════"
    
    original_size=$(stat -f%z "$input_file" 2>/dev/null || stat -c%s "$input_file")
    echo "Original size: $original_size bytes"
    echo ""
    
    # Test each algorithm
    for algo in huffman lz4 zstd dictionary rle; do
        output_file="test-results/${filename%.json}-${algo}.owlsol"
        
        # Compress
        ./target/release/owlsol compress -i "$input_file" -a "$algo" -o "$output_file" 2>/dev/null
        
        if [ -f "$output_file" ]; then
            compressed_size=$(stat -f%z "$output_file" 2>/dev/null || stat -c%s "$output_file")
            ratio=$(echo "scale=2; (1 - $compressed_size / $original_size) * 100" | bc)
            
            printf "%-12s | %8d bytes | %6.2f%% saved\n" "$algo" "$compressed_size" "$ratio"
        else
            printf "%-12s | FAILED\n" "$algo"
        fi
    done

done

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "✅ Test Complete! Results saved in: test-results/"
echo "═══════════════════════════════════════════════════════════════"
