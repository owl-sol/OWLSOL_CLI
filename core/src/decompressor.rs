use crate::algorithms::{dict_decompress, rle_decompress, HuffmanCodec};
use crate::error::{CompressionError, Result};
use crate::metadata::{CompressionAlgorithm, CompressionMetadata};
use crate::utils::verify_checksum;

pub struct Decompressor;

impl Decompressor {
    pub fn new() -> Self {
        Self
    }

    pub fn decompress(&self, data: &[u8], metadata: &CompressionMetadata) -> Result<Vec<u8>> {
        // Validate metadata
        if !metadata.validate() {
            return Err(CompressionError::InvalidMetadata(
                "Invalid compression metadata".into(),
            ));
        }

        // Verify checksum if available
        if metadata.checksum != 0 && !verify_checksum(data, metadata.checksum) {
            return Err(CompressionError::CorruptedData(
                "Checksum mismatch".into(),
            ));
        }

        // Decompress based on algorithm
        let decompressed = match metadata.algorithm {
            CompressionAlgorithm::None => data.to_vec(),
            CompressionAlgorithm::Huffman => self.decompress_huffman(data, metadata)?,
            CompressionAlgorithm::Dictionary => self.decompress_dictionary(data)?,
            CompressionAlgorithm::RunLength => self.decompress_rle(data)?,
            CompressionAlgorithm::Hybrid => {
                return Err(CompressionError::UnsupportedAlgorithm(
                    "Hybrid algorithm should be stored as specific algorithm".into(),
                ))
            }
        };

        // Verify decompressed size
        if decompressed.len() != metadata.original_size as usize {
            return Err(CompressionError::CorruptedData(format!(
                "Size mismatch: expected {}, got {}",
                metadata.original_size,
                decompressed.len()
            )));
        }

        Ok(decompressed)
    }

    fn decompress_huffman(&self, data: &[u8], metadata: &CompressionMetadata) -> Result<Vec<u8>> {
        if data.len() < 4 {
            return Err(CompressionError::InsufficientData);
        }

        // Read tree size
        let tree_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        
        if data.len() < 4 + tree_size {
            return Err(CompressionError::decompression_failed("Truncated tree data"));
        }

        // Deserialize tree
        let mut codec = HuffmanCodec::new();
        codec.deserialize_tree(&data[4..4 + tree_size])?;

        // Decode data
        let encoded = &data[4 + tree_size..];
        codec.decode(encoded, metadata.original_size as usize)
    }

    fn decompress_dictionary(&self, data: &[u8]) -> Result<Vec<u8>> {
        dict_decompress(data)
    }

    fn decompress_rle(&self, data: &[u8]) -> Result<Vec<u8>> {
        rle_decompress(data)
    }
}

impl Default for Decompressor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Compressor;

    #[test]
    fn test_roundtrip_huffman() {
        let compressor = Compressor::new();
        let decompressor = Decompressor::new();
        let data = b"hello world hello world hello";

        let result = compressor
            .compress_with_algorithm(data, Some(CompressionAlgorithm::Huffman))
            .unwrap();
        let decompressed = decompressor.decompress(&result.data, &result.metadata).unwrap();

        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_roundtrip_rle() {
        let compressor = Compressor::new();
        let decompressor = Decompressor::new();
        let data = vec![b'A'; 1000];

        let result = compressor
            .compress_with_algorithm(&data, Some(CompressionAlgorithm::RunLength))
            .unwrap();
        let decompressed = decompressor.decompress(&result.data, &result.metadata).unwrap();

        assert_eq!(data, decompressed);
    }

    #[test]
    fn test_roundtrip_dictionary() {
        let compressor = Compressor::new();
        let decompressor = Decompressor::new();
        let data = b"test data test data test data";

        let result = compressor
            .compress_with_algorithm(data, Some(CompressionAlgorithm::Dictionary))
            .unwrap();
        let decompressed = decompressor.decompress(&result.data, &result.metadata).unwrap();

        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_invalid_metadata() {
        let decompressor = Decompressor::new();
        let mut metadata = CompressionMetadata::new(CompressionAlgorithm::None, 100, 50);
        metadata.version = 99; // Invalid version

        let result = decompressor.decompress(b"data", &metadata);
        assert!(result.is_err());
    }
}
