use owlsol_core::{CompressionMetadata, CompressionResult};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub struct CompressedAccount {
    pub address: Pubkey,
    pub data: Vec<u8>,
    pub metadata: CompressionMetadata,
    pub rent: u64,
}

impl CompressedAccount {
    pub fn new(address: Pubkey, result: CompressionResult, rent: u64) -> Self {
        Self {
            address,
            data: result.data,
            metadata: result.metadata,
            rent,
        }
    }

    pub fn savings(&self) -> u64 {
        let original_rent =
            self.rent * self.metadata.original_size / self.metadata.compressed_size.max(1);
        original_rent.saturating_sub(self.rent)
    }

    pub fn savings_sol(&self) -> f64 {
        self.savings() as f64 / 1_000_000_000.0
    }

    pub fn rent_sol(&self) -> f64 {
        self.rent as f64 / 1_000_000_000.0
    }

    pub fn original_rent(&self) -> u64 {
        self.rent * self.metadata.original_size / self.metadata.compressed_size.max(1)
    }

    pub fn original_rent_sol(&self) -> f64 {
        self.original_rent() as f64 / 1_000_000_000.0
    }

    pub fn compression_details(&self) -> String {
        format!(
            "Algorithm: {}\nOriginal: {} bytes\nCompressed: {} bytes\nRatio: {:.2}%\nRent: {:.6} SOL\nSavings: {:.6} SOL",
            self.metadata.algorithm.as_str(),
            self.metadata.original_size,
            self.metadata.compressed_size,
            self.metadata.compression_percentage(),
            self.rent_sol(),
            self.savings_sol()
        )
    }
}
