use crate::analyzer::DataAnalysis;
use crate::metadata::CompressionAlgorithm;

pub struct AlgorithmSelector;

impl AlgorithmSelector {
    pub fn new() -> Self {
        Self
    }

    pub fn select_algorithm(&self, analysis: &DataAnalysis) -> CompressionAlgorithm {
        // Random/encrypted data - don't compress
        if analysis.is_random() {
            return CompressionAlgorithm::None;
        }

        // Many repeated sequences - use dictionary
        if analysis.has_patterns() && !analysis.has_runs() {
            return CompressionAlgorithm::Dictionary;
        }

        // Long runs - use RLE
        if analysis.has_runs() {
            return CompressionAlgorithm::RunLength;
        }

        // Text-like data with varied symbols - use Huffman
        if analysis.is_text_like() {
            return CompressionAlgorithm::Huffman;
        }

        // Mixed patterns - try hybrid approach
        CompressionAlgorithm::Hybrid
    }

    pub fn estimate_compression_ratio(&self, analysis: &DataAnalysis) -> f64 {
        let algo = self.select_algorithm(analysis);
        
        match algo {
            CompressionAlgorithm::None => 1.0,
            CompressionAlgorithm::Huffman => {
                // Entropy-based estimate
                let bits_per_byte = analysis.entropy;
                (bits_per_byte / 8.0).max(0.4)
            }
            CompressionAlgorithm::RunLength => {
                // Run-based estimate
                if analysis.max_run_length > 0 {
                    let factor = analysis.avg_run_length.max(1.0);
                    (1.0 / factor).max(0.2)
                } else {
                    0.8
                }
            }
            CompressionAlgorithm::Dictionary => {
                // Pattern-based estimate
                if analysis.repeated_sequences > 0 {
                    0.5
                } else {
                    0.8
                }
            }
            CompressionAlgorithm::Hybrid => 0.4,
            CompressionAlgorithm::Lz4 => 0.5,
            CompressionAlgorithm::Zstd => 0.4,
        }
    }

    pub fn should_compress(&self, analysis: &DataAnalysis) -> bool {
        let ratio = self.estimate_compression_ratio(analysis);
        ratio < 0.95 // Only compress if saves at least 5%
    }
}

impl Default for AlgorithmSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_selection() {
        let selector = AlgorithmSelector::new();
        
        // Test with different data characteristics
        let mut analysis = DataAnalysis {
            size: 1000,
            entropy: 7.8,
            unique_bytes: 200,
            max_run_length: 2,
            avg_run_length: 1.1,
            repeated_sequences: 5,
            byte_distribution: Default::default(),
        };

        assert_eq!(selector.select_algorithm(&analysis), CompressionAlgorithm::None);

        analysis.entropy = 5.0;
        analysis.unique_bytes = 100;
        assert_eq!(selector.select_algorithm(&analysis), CompressionAlgorithm::Huffman);

        analysis.max_run_length = 50;
        analysis.avg_run_length = 10.0;
        assert_eq!(selector.select_algorithm(&analysis), CompressionAlgorithm::RunLength);
    }
}
