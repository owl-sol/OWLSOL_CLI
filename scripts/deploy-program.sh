#!/bin/bash
set -e

echo "🦉 OWLSOL Program Deployment"
echo "============================"
echo ""

# Check if Anchor is installed
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor not found. Please install Anchor:"
    echo "   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force"
    exit 1
fi

echo "✓ Anchor installed"
echo ""

# Navigate to program directory
cd program

# Build program
echo "Building Solana program..."
anchor build

echo "✓ Program built"
echo ""

# Deploy to devnet
echo "Deploying to devnet..."
anchor deploy --provider.cluster devnet

echo ""
echo "✅ Program deployed successfully!"
echo ""
echo "Program ID saved in: program/target/deploy/owlsol-keypair.json"
echo ""
echo "Next steps:"
echo "  1. Update program ID in Anchor.toml"
echo "  2. Test with: anchor test"
