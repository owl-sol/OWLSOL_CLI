use crate::algorithms::{dict_compress, rle_compress, HuffmanCodec};
use crate::analyzer::DataAnalyzer;
use crate::error::{CompressionError, Result};
use crate::metadata::{
    CompressionAlgorithm, CompressionMetadata, CompressionResult, MAX_DATA_SIZE,
};
use crate::selector::AlgorithmSelector;
use crate::utils::calculate_checksum;

pub struct Compressor {
    analyzer: DataAnalyzer,
    selector: AlgorithmSelector,
}

impl Compressor {
    fn compress_lz4(&self, data: &[u8]) -> Result<(Vec<u8>, CompressionAlgorithm)> {
        use lz4::block::compress;
        let compressed = compress(data, None, false)
            .map_err(|e| CompressionError::compression_failed(format!("LZ4: {}", e)))?;
        Ok((compressed, CompressionAlgorithm::None)) // Will set algo below
    }

    fn compress_zstd(&self, data: &[u8]) -> Result<(Vec<u8>, CompressionAlgorithm)> {
        use zstd::stream::encode_all;
        use std::io::Cursor;
        let compressed = encode_all(Cursor::new(data), 1)
            .map_err(|e| CompressionError::compression_failed(format!("Zstd: {}", e)))?;
        Ok((compressed, CompressionAlgorithm::None)) // Will set algo below
    }
    pub fn new() -> Self {
        Self {
            analyzer: DataAnalyzer::new(),
            selector: AlgorithmSelector::new(),
        }
    }

    pub fn compress(&self, data: &[u8]) -> Result<CompressionResult> {
        self.compress_with_algorithm(data, None)
    }

    pub fn compress_with_algorithm(
        &self,
        data: &[u8],
        algorithm: Option<CompressionAlgorithm>,
    ) -> Result<CompressionResult> {
        // Validation
        if data.is_empty() {
            return Err(CompressionError::invalid_input("Empty data"));
        }

        if data.len() > MAX_DATA_SIZE {
            return Err(CompressionError::DataTooLarge(data.len(), MAX_DATA_SIZE));
        }

        let original_size = data.len() as u64;

        // If algorithm is specified, use it. If None (auto), try all and pick best.
        let algorithms = match algorithm {
            Some(a) => vec![a],
            None => vec![
                CompressionAlgorithm::Huffman,
                CompressionAlgorithm::Dictionary,
                CompressionAlgorithm::RunLength,
                CompressionAlgorithm::Hybrid,
            ],
        };

        let mut best_result: Option<CompressionResult> = None;
        // Try all custom algorithms
        for algo in algorithms {
            let (compressed_data, actual_algo) = match algo {
                CompressionAlgorithm::None => (data.to_vec(), CompressionAlgorithm::None),
                CompressionAlgorithm::Huffman => self.compress_huffman(data)?,
                CompressionAlgorithm::Dictionary => self.compress_dictionary(data)?,
                CompressionAlgorithm::RunLength => self.compress_rle(data)?,
                CompressionAlgorithm::Hybrid => self.compress_hybrid(data)?,
            };
            let compressed_size = compressed_data.len() as u64;
            if compressed_size < original_size {
                let checksum = calculate_checksum(&compressed_data);
                let metadata = CompressionMetadata::new(actual_algo, original_size, compressed_size)
                    .with_checksum(checksum);
                let result = CompressionResult::new(compressed_data, metadata);
                if best_result.is_none() || compressed_size < best_result.as_ref().unwrap().metadata.compressed_size {
                    best_result = Some(result);
                }
            }
        }
        // Try LZ4
        if let Ok((lz4_data, _)) = self.compress_lz4(data) {
            let lz4_size = lz4_data.len() as u64;
            if lz4_size < original_size {
                let checksum = calculate_checksum(&lz4_data);
                let metadata = CompressionMetadata::new(CompressionAlgorithm::None, original_size, lz4_size)
                    .with_checksum(checksum);
                let result = CompressionResult::new(lz4_data, metadata);
                if best_result.is_none() || lz4_size < best_result.as_ref().unwrap().metadata.compressed_size {
                    best_result = Some(result);
                }
            }
        }
        // Try Zstd
        if let Ok((zstd_data, _)) = self.compress_zstd(data) {
            let zstd_size = zstd_data.len() as u64;
            if zstd_size < original_size {
                let checksum = calculate_checksum(&zstd_data);
                let metadata = CompressionMetadata::new(CompressionAlgorithm::None, original_size, zstd_size)
                    .with_checksum(checksum);
                let result = CompressionResult::new(zstd_data, metadata);
                if best_result.is_none() || zstd_size < best_result.as_ref().unwrap().metadata.compressed_size {
                    best_result = Some(result);
                }
            }
        }
        // If no worthwhile compression, return original data with None
        let final_result = best_result.unwrap_or_else(|| {
            let checksum = calculate_checksum(data);
            let metadata = CompressionMetadata::new(CompressionAlgorithm::None, original_size, original_size)
                .with_checksum(checksum);
            CompressionResult::new(data.to_vec(), metadata)
        });
        Ok(final_result)
    }

    fn compress_huffman(&self, data: &[u8]) -> Result<(Vec<u8>, CompressionAlgorithm)> {
        let mut codec = HuffmanCodec::new();
        codec.build_from_data(data)?;

        let encoded = codec.encode(data)?;
        let tree = codec.serialize_tree()?;

        // Format: [tree_size(4 bytes)][tree][encoded_data]
        let tree_size = tree.len() as u32;
        let mut result = Vec::with_capacity(4 + tree.len() + encoded.len());
        result.extend_from_slice(&tree_size.to_le_bytes());
        result.extend_from_slice(&tree);
        result.extend_from_slice(&encoded);

        Ok((result, CompressionAlgorithm::Huffman))
    }

    fn compress_dictionary(&self, data: &[u8]) -> Result<(Vec<u8>, CompressionAlgorithm)> {
        let compressed = dict_compress(data)?;
        Ok((compressed, CompressionAlgorithm::Dictionary))
    }

    fn compress_rle(&self, data: &[u8]) -> Result<(Vec<u8>, CompressionAlgorithm)> {
        let compressed = rle_compress(data)?;
        Ok((compressed, CompressionAlgorithm::RunLength))
    }

    fn compress_hybrid(&self, data: &[u8]) -> Result<(Vec<u8>, CompressionAlgorithm)> {
        // Try all algorithms and pick the best
        let results = vec![
            self.compress_huffman(data).ok(),
            self.compress_dictionary(data).ok(),
            self.compress_rle(data).ok(),
        ];

        let mut best = (data.to_vec(), CompressionAlgorithm::None);
        let mut best_size = data.len();

        for result in results.into_iter().flatten() {
            if result.0.len() < best_size {
                best = result;
                best_size = best.0.len();
            }
        }

        Ok(best)
    }

    pub fn estimate_ratio(&self, data: &[u8]) -> f64 {
        let analysis = self.analyzer.analyze(data);
        self.selector.estimate_compression_ratio(&analysis)
    }

    pub fn analyze_data(&self, data: &[u8]) -> crate::analyzer::DataAnalysis {
        self.analyzer.analyze(data)
    }
}

impl Default for Compressor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_compression() {
        let compressor = Compressor::new();
        let data = b"hello world hello world";
        let result = compressor.compress(data).unwrap();
        
        assert!(result.metadata.compressed_size <= result.metadata.original_size);
        assert!(result.metadata.validate());
    }

    #[test]
    fn test_empty_data() {
        let compressor = Compressor::new();
        assert!(compressor.compress(b"").is_err());
    }

    #[test]
    fn test_large_data() {
        let compressor = Compressor::new();
        let data = vec![0u8; MAX_DATA_SIZE + 1];
        assert!(compressor.compress(&data).is_err());
    }

    #[test]
    fn test_algorithm_selection() {
        let compressor = Compressor::new();
        
        // RLE-friendly data
        let data = vec![b'A'; 1000];
        let result = compressor.compress(&data).unwrap();
        assert_eq!(result.metadata.algorithm, CompressionAlgorithm::RunLength);
        assert!(result.metadata.compression_percentage() > 90.0);
    }

    #[test]
    fn test_incompressible_data() {
        let compressor = Compressor::new();
        let data: Vec<u8> = (0..1000).map(|i| (i * 7919) as u8).collect();
        let result = compressor.compress(&data).unwrap();
        
        // Should not compress random data
        assert_eq!(result.metadata.algorithm, CompressionAlgorithm::None);
    }
}
