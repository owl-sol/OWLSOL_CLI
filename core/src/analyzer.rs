use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DataAnalysis {
    pub size: usize,
    pub entropy: f64,
    pub unique_bytes: usize,
    pub max_run_length: usize,
    pub avg_run_length: f64,
    pub repeated_sequences: usize,
    pub byte_distribution: HashMap<u8, usize>,
}

impl DataAnalysis {
    pub fn is_random(&self) -> bool {
        self.entropy > 7.5
    }

    pub fn has_runs(&self) -> bool {
        self.max_run_length > 10 || self.avg_run_length > 3.0
    }

    pub fn has_patterns(&self) -> bool {
        self.repeated_sequences > self.size / 20
    }

    pub fn is_text_like(&self) -> bool {
        self.unique_bytes < 128 && self.entropy < 7.0
    }
}

pub struct DataAnalyzer;

impl DataAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, data: &[u8]) -> DataAnalysis {
        let size = data.len();
        let byte_distribution = self.count_bytes(data);
        let entropy = self.calculate_entropy(&byte_distribution, size);
        let unique_bytes = byte_distribution.len();
        let (max_run, avg_run) = self.analyze_runs(data);
        let repeated = self.count_repeated_sequences(data);

        DataAnalysis {
            size,
            entropy,
            unique_bytes,
            max_run_length: max_run,
            avg_run_length: avg_run,
            repeated_sequences: repeated,
            byte_distribution,
        }
    }

    fn count_bytes(&self, data: &[u8]) -> HashMap<u8, usize> {
        let mut counts = HashMap::new();
        for &byte in data {
            *counts.entry(byte).or_insert(0) += 1;
        }
        counts
    }

    fn calculate_entropy(&self, distribution: &HashMap<u8, usize>, total: usize) -> f64 {
        let total_f = total as f64;
        distribution
            .values()
            .map(|&count| {
                let p = count as f64 / total_f;
                if p > 0.0 {
                    -p * p.log2()
                } else {
                    0.0
                }
            })
            .sum()
    }

    fn analyze_runs(&self, data: &[u8]) -> (usize, f64) {
        if data.is_empty() {
            return (0, 0.0);
        }

        let mut max_run = 1;
        let mut total_runs = 0;
        let mut run_count = 0;
        let mut current_run = 1;

        for i in 1..data.len() {
            if data[i] == data[i - 1] {
                current_run += 1;
            } else {
                if current_run > 1 {
                    max_run = max_run.max(current_run);
                    total_runs += current_run;
                    run_count += 1;
                }
                current_run = 1;
            }
        }

        if current_run > 1 {
            max_run = max_run.max(current_run);
            total_runs += current_run;
            run_count += 1;
        }

        let avg = if run_count > 0 {
            total_runs as f64 / run_count as f64
        } else {
            0.0
        };

        (max_run, avg)
    }

    fn count_repeated_sequences(&self, data: &[u8]) -> usize {
        let window = 4.min(data.len());
        let mut sequences = HashMap::new();

        for i in 0..data.len().saturating_sub(window - 1) {
            let seq = &data[i..i + window];
            *sequences.entry(seq).or_insert(0) += 1;
        }

        sequences.values().filter(|&&count| count > 1).count()
    }
}

impl Default for DataAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_data_detection() {
        let data: Vec<u8> = (0..1000).map(|i| (i * 7919) as u8).collect();
        let analyzer = DataAnalyzer::new();
        let analysis = analyzer.analyze(&data);
        assert!(analysis.is_random());
    }

    #[test]
    fn test_run_detection() {
        let data = vec![b'A'; 100];
        let analyzer = DataAnalyzer::new();
        let analysis = analyzer.analyze(&data);
        assert!(analysis.has_runs());
        assert_eq!(analysis.max_run_length, 100);
    }

    #[test]
    fn test_pattern_detection() {
        let data = b"repeatrepeatrepeatrepeat";
        let analyzer = DataAnalyzer::new();
        let analysis = analyzer.analyze(data);
        assert!(analysis.has_patterns());
    }
}
