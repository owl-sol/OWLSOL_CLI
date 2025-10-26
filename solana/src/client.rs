use crate::error::{Result, SolanaError};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::path::Path;

pub struct SolanaClient {
    rpc: RpcClient,
    payer: Keypair,
}

impl SolanaClient {
    pub fn new<P: AsRef<Path>>(rpc_url: &str, keypair_path: P) -> Result<Self> {
        let rpc = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );

        let keypair_bytes = std::fs::read(keypair_path)?;
        let payer = Keypair::from_bytes(&keypair_bytes)
            .map_err(|_| SolanaError::InvalidKeypair)?;

        Ok(Self { rpc, payer })
    }

    pub fn devnet() -> Result<Self> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let keypair_path = format!("{}/.config/solana/id.json", home);
        Self::new("https://api.devnet.solana.com", keypair_path)
    }

    pub fn testnet() -> Result<Self> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let keypair_path = format!("{}/.config/solana/id.json", home);
        Self::new("https://api.testnet.solana.com", keypair_path)
    }

    pub fn get_balance(&self) -> Result<u64> {
        self.rpc
            .get_balance(&self.payer.pubkey())
            .map_err(|e| SolanaError::RpcError(e.to_string()))
    }

    pub fn get_balance_sol(&self) -> Result<f64> {
        Ok(self.get_balance()? as f64 / 1_000_000_000.0)
    }

    pub fn airdrop(&self, lamports: u64) -> Result<()> {
        let signature = self
            .rpc
            .request_airdrop(&self.payer.pubkey(), lamports)
            .map_err(|e| SolanaError::RpcError(e.to_string()))?;

        // Wait for confirmation
        loop {
            if let Ok(confirmed) = self.rpc.confirm_transaction(&signature) {
                if confirmed {
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        Ok(())
    }

    pub fn airdrop_if_needed(&self, min_balance: u64) -> Result<()> {
        let balance = self.get_balance()?;
        if balance < min_balance {
            let amount = min_balance - balance + 1_000_000_000; // Add 1 SOL buffer
            self.airdrop(amount)?;
        }
        Ok(())
    }

    pub fn pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }

    pub fn calculate_rent(&self, data_len: usize) -> Result<u64> {
        self.rpc
            .get_minimum_balance_for_rent_exemption(data_len)
            .map_err(|e| SolanaError::RpcError(e.to_string()))
    }

    pub fn calculate_rent_sol(&self, data_len: usize) -> Result<f64> {
        Ok(self.calculate_rent(data_len)? as f64 / 1_000_000_000.0)
    }

    pub fn get_latest_blockhash(&self) -> Result<solana_sdk::hash::Hash> {
        self.rpc
            .get_latest_blockhash()
            .map_err(|e| SolanaError::RpcError(e.to_string()))
    }

    pub fn send_transaction(&self, transaction: &Transaction) -> Result<String> {
        self.rpc
            .send_and_confirm_transaction(transaction)
            .map(|sig| sig.to_string())
            .map_err(|e| SolanaError::TransactionFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires devnet connection
    fn test_devnet_connection() {
        let client = SolanaClient::devnet();
        assert!(client.is_ok());
    }

    #[test]
    #[ignore]
    fn test_get_balance() {
        let client = SolanaClient::devnet().unwrap();
        let balance = client.get_balance();
        assert!(balance.is_ok());
    }
}
