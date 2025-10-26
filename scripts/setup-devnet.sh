#!/bin/bash
set -e

echo "🦉 OWLSOL Devnet Setup"
echo "====================="
echo ""

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI not found. Installing..."
    sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
fi

echo "✓ Solana CLI installed"
echo ""

# Configure for devnet
echo "Configuring for devnet..."
solana config set --url https://api.devnet.solana.com

# Generate keypair if it doesn't exist
if [ ! -f "$HOME/.config/solana/id.json" ]; then
    echo "Generating new keypair..."
    solana-keygen new --no-bip39-passphrase
fi

echo "✓ Keypair configured"
echo ""

# Get wallet address
WALLET=$(solana address)
echo "Wallet address: $WALLET"
echo ""

# Check balance
BALANCE=$(solana balance)
echo "Current balance: $BALANCE"

# Airdrop if needed
if [[ "$BALANCE" == "0 SOL" ]]; then
    echo ""
    echo "Requesting airdrop..."
    solana airdrop 2
    echo "✓ Airdrop received"
fi

echo ""
echo "✅ Devnet setup complete!"
echo ""
echo "Next steps:"
echo "  1. Build the project: cargo build --release"
echo "  2. Run compression: ./target/release/owlsol compress -i <file>"
echo "  3. Deploy to devnet: ./target/release/owlsol compress -i <file> --deploy"
