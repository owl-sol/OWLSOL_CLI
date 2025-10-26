use anyhow::Result;
use solana_sdk::signature::Keypair;
use std::path::PathBuf;

/// Load wallet from standard Solana CLI location
pub fn load_wallet() -> Result<Keypair> {
    let wallet_path = get_wallet_path();
    let keypair = solana_sdk::signature::read_keypair_file(&wallet_path)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(keypair)
}

/// Get default wallet path
pub fn get_wallet_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".config")
        .join("solana")
        .join("id.json")
}

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub name: String,
    pub rpc_url: String,
}

impl NetworkConfig {
    pub fn mainnet() -> Self {
        Self {
            name: "mainnet".to_string(),
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
        }
    }

    pub fn devnet() -> Self {
        Self {
            name: "devnet".to_string(),
            rpc_url: "https://api.devnet.solana.com".to_string(),
        }
    }

    pub fn testnet() -> Self {
        Self {
            name: "testnet".to_string(),
            rpc_url: "https://api.testnet.solana.com".to_string(),
        }
    }
}
