#!/bin/bash

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║              Performance Benchmark Suite                      ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

if [ ! -d "solana-accounts" ]; then
    echo "❌ Error: solana-accounts directory not found"
    exit 1
fi

ITERATIONS=1000
REPORT_FILE="benchmark-report-$(date +%Y%m%d-%H%M%S).txt"

echo "Running $ITERATIONS iterations per file..."
echo "Report will be saved to: $REPORT_FILE"
echo ""

{
    echo "OWLSOL CLI - Performance Benchmark Report"
    echo "========================================"
    echo "Date: $(date)"
    echo "Iterations: $ITERATIONS"
    echo ""
    
    for input_file in solana-accounts/*.json; do
        filename=$(basename "$input_file")
        echo ""
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo "File: $filename"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        
        ./target/release/owlsol benchmark -i "$input_file" -n "$ITERATIONS"
        
        echo ""
    done
    
    echo ""
    echo "========================================"
    echo "Benchmark Complete"
    echo "========================================"
} | tee "$REPORT_FILE"

echo ""
echo "✅ Report saved to: $REPORT_FILE"
