//! OWLSOL Core - Compression Engine
//!
//! Provides hybrid compression algorithms optimized for Solana account storage.

pub mod algorithms;
pub mod analyzer;
pub mod compressor;
pub mod decompressor;
pub mod error;
pub mod metadata;
pub mod selector;
pub mod utils;

pub use compressor::Compressor;
pub use decompressor::Decompressor;
pub use error::{CompressionError, Result};
pub use metadata::{CompressionAlgorithm, CompressionMetadata, CompressionResult};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        CompressionAlgorithm, CompressionError, CompressionMetadata, CompressionResult, Compressor,
        Decompressor, Result,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_compression() {
        let compressor = Compressor::new();
        let data = b"hello world";
        let result = compressor.compress(data);
        assert!(result.is_ok());
    }
}
