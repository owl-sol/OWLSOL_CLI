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
        Ok((compressed, CompressionAlgorithm::Lz4))
    }

    fn compress_zstd(&self, data: &[u8]) -> Result<(Vec<u8>, CompressionAlgorithm)> {
        use std::io::Cursor;
        use zstd::stream::encode_all;
        let compressed = encode_all(Cursor::new(data), 1)
            .map_err(|e| CompressionError::compression_failed(format!("Zstd: {}", e)))?;
        Ok((compressed, CompressionAlgorithm::Zstd))
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

        // If specific algorithm requested, use only that
        if let Some(algo) = algorithm {
            let (compressed_data, actual_algo) = match algo {
                CompressionAlgorithm::None => (data.to_vec(), CompressionAlgorithm::None),
                CompressionAlgorithm::Huffman => self.compress_huffman(data)?,
                CompressionAlgorithm::Dictionary => self.compress_dictionary(data)?,
                CompressionAlgorithm::RunLength => self.compress_rle(data)?,
                CompressionAlgorithm::Lz4 => self.compress_lz4(data)?,
                CompressionAlgorithm::Zstd => self.compress_zstd(data)?,
                CompressionAlgorithm::Hybrid => self.compress_hybrid(data)?,
            };

            let compressed_size = compressed_data.len() as u64;
            // Check if compression is worthwhile
            let (final_data, final_algo) = if compressed_size >= original_size {
                (data.to_vec(), CompressionAlgorithm::None)
            } else {
                (compressed_data, actual_algo)
            };

            let checksum = calculate_checksum(&final_data);
            let metadata =
                CompressionMetadata::new(final_algo, original_size, final_data.len() as u64)
                    .with_checksum(checksum);

            return Ok(CompressionResult::new(final_data, metadata));
        }

        // Auto mode: Try all algorithms and pick best
        let mut best_result: Option<(Vec<u8>, CompressionAlgorithm, u64)> = None;

        // Try all algorithms
        let results = vec![
            self.compress_huffman(data).ok(),
            self.compress_dictionary(data).ok(),
            self.compress_rle(data).ok(),
            self.compress_lz4(data).ok(),
            self.compress_zstd(data).ok(),
        ];

        for result in results.into_iter().flatten() {
            let compressed_size = result.0.len() as u64;
            if compressed_size < original_size
                && (best_result.is_none() || compressed_size < best_result.as_ref().unwrap().2)
            {
                best_result = Some((result.0, result.1, compressed_size));
            }
        }

        // Use best result or original data
        let (final_data, final_algo) = if let Some((data, algo, _)) = best_result {
            (data, algo)
        } else {
            (data.to_vec(), CompressionAlgorithm::None)
        };

        let checksum = calculate_checksum(&final_data);
        let metadata = CompressionMetadata::new(final_algo, original_size, final_data.len() as u64)
            .with_checksum(checksum);

        Ok(CompressionResult::new(final_data, metadata))
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
        println!(
            "[COMPRESS HUFFMAN] Original len: {}, Compressed len: {}",
            data.len(),
            result.len()
        );
        Ok((result, CompressionAlgorithm::Huffman))
    }

    fn compress_dictionary(&self, data: &[u8]) -> Result<(Vec<u8>, CompressionAlgorithm)> {
        let compressed = dict_compress(data)?;
        println!(
            "[COMPRESS DICT] Original len: {}, Compressed len: {}",
            data.len(),
            compressed.len()
        );
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
        // For random data, either None, Lz4, or Zstd may be chosen depending on output size
        match result.metadata.algorithm {
            CompressionAlgorithm::None | CompressionAlgorithm::Lz4 | CompressionAlgorithm::Zstd => {
            }
            other => panic!("Unexpected algorithm for incompressible data: {:?}", other),
        }
    }
}
