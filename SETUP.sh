#!/bin/bash

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║            🦉  OWLSOL CLI - Initial Setup  🦉                ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Create scripts directory
mkdir -p scripts

echo "📝 Step 1: Making all scripts executable..."
chmod +x scripts/*.sh 2>/dev/null

echo "🔨 Step 2: Building OWLSOL CLI (Release Mode)..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed. Please check your Rust installation."
    exit 1
fi

echo ""
echo "📦 Step 3: Generating test data..."
./scripts/generate-solana-accounts.sh

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                    🎉 Setup Complete! 🎉                     ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Available commands:"
echo "  • ./scripts/full-demo.sh          - Run complete demo"
echo "  • ./scripts/quick-test.sh         - Quick functionality test"
echo "  • ./scripts/test-all-algorithms.sh - Compare all algorithms"
echo "  • ./scripts/benchmark-all.sh      - Performance benchmarks"
echo "  • ./scripts/cleanup.sh            - Clean up test files"
echo ""
echo "Or use the CLI directly:"
echo "  • ./target/release/owlsol --help"
echo ""
echo "Happy compressing! 🦉"
