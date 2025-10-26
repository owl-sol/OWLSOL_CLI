#!/bin/bash

echo "🦉 Generating OWLSOL Test Data..."
echo ""

mkdir -p test-data

# 1. NFT Metadata (Realistic Solana NFT)
cat > test-data/nft-metadata.json << 'EOF'
{
  "name": "Degen Ape #4269",
  "symbol": "DAPE",
  "description": "A collection of 10,000 unique Degen Apes living on the Solana blockchain",
  "seller_fee_basis_points": 500,
  "image": "https://arweave.net/26YdhY_eAzv26YdhY1uu9uiA3nmDZYwP8MwZAultcE?ext=jpeg",
  "external_url": "https://degenape.academy",
  "attributes": [
    {
      "trait_type": "Background",
      "value": "Blue"
    },
    {
      "trait_type": "Fur",
      "value": "Golden"
    },
    {
      "trait_type": "Eyes",
      "value": "Laser Eyes"
    },
    {
      "trait_type": "Mouth",
      "value": "Grin"
    },
    {
      "trait_type": "Clothes",
      "value": "Tuxedo"
    },
    {
      "trait_type": "Hat",
      "value": "Crown"
    },
    {
      "trait_type": "Rarity",
      "value": "Legendary"
    }
  ],
  "properties": {
    "files": [
      {
        "uri": "https://arweave.net/26YdhY_eAzv26YdhY1uu9uiA3nmDZYwP8MwZAultcE?ext=jpeg",
        "type": "image/jpeg"
      }
    ],
    "category": "image",
    "creators": [
      {
        "address": "D3XrkNZz6wx6cofot7Zohsf2KSsu2ArngNk8VqU9cTY",
        "share": 100
      }
    ]
  },
  "collection": {
    "name": "Degen Ape Academy",
    "family": "Degen Apes"
  }
}
EOF

# 2. Game Player Profile
cat > test-data/game-profile.json << 'EOF'
{
  "player_id": "7xKSWqU9cTYD3Xrk",
  "username": "CryptoWarrior420",
  "level": 87,
  "xp": 245680,
  "health": 850,
  "mana": 420,
  "inventory": [
    {"item_id": 1001, "name": "Legendary Sword", "quantity": 1, "equipped": true},
    {"item_id": 2045, "name": "Health Potion", "quantity": 15, "equipped": false},
    {"item_id": 3012, "name": "Magic Shield", "quantity": 1, "equipped": true},
    {"item_id": 4567, "name": "Speed Boots", "quantity": 1, "equipped": true},
    {"item_id": 5789, "name": "Ancient Amulet", "quantity": 1, "equipped": true}
  ],
  "achievements": [
    "First Kill", "Level 50", "Dragon Slayer", "PvP Master", "Guild Leader"
  ],
  "stats": {
    "battles_won": 1547,
    "battles_lost": 423,
    "kills": 8934,
    "deaths": 1205,
    "gold_earned": 45678900,
    "quests_completed": 342
  },
  "last_login": "2025-10-26T12:34:56Z",
  "created_at": "2024-01-15T08:22:10Z"
}
EOF

# 3. DeFi Protocol State
cat > test-data/defi-state.json << 'EOF'
{
  "protocol": "OwlSwap",
  "version": "2.1.0",
  "pools": [
    {
      "pool_id": "SOL-USDC",
      "token_a": "So11111111111111111111111111111111111111112",
      "token_b": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
      "liquidity": 12500000.50,
      "volume_24h": 3456789.25,
      "fees_24h": 10370.37,
      "apy": 23.45
    },
    {
      "pool_id": "SOL-BONK",
      "token_a": "So11111111111111111111111111111111111111112",
      "token_b": "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
      "liquidity": 8750000.00,
      "volume_24h": 1234567.89,
      "fees_24h": 3703.70,
      "apy": 18.92
    }
  ],
  "total_value_locked": 21250000.50,
  "total_volume_24h": 4691357.14,
  "users": 15432,
  "transactions_24h": 45678
}
EOF

# 4. Social Media Post
cat > test-data/social-post.json << 'EOF'
{
  "post_id": "post_xyz789",
  "author": "solana_dev_123",
  "author_wallet": "7xKSWqU9cTYD3XrkNZz6wx6cofot7Zohsf2KSsu2ArngNk",
  "content": "Just deployed my first Solana program! 🚀 Loving the speed and low fees. Who else is building on Solana? #Solana #Web3 #Blockchain",
  "timestamp": "2025-10-26T15:45:30Z",
  "likes": 342,
  "retweets": 89,
  "replies": 45,
  "media": [
    {
      "type": "image",
      "url": "https://arweave.net/abc123def456",
      "width": 1920,
      "height": 1080
    }
  ],
  "hashtags": ["Solana", "Web3", "Blockchain"],
  "mentions": ["@solana", "@anchor_lang"]
}
EOF

# 5. Large Text File (for RLE testing)
echo "Generating large text file..."
for i in {1..1000}; do
    echo "AAAAAAAAAABBBBBBBBBBCCCCCCCCCCDDDDDDDDDDEEEEEEEEEE" >> test-data/repeated-text.txt
done

# 6. Random Binary Data (incompressible)
echo "Generating random binary data..."
dd if=/dev/urandom of=test-data/random-data.bin bs=1024 count=10 2>/dev/null

echo ""
echo "✅ Test data generated in test-data/"
echo ""
ls -lh test-data/
