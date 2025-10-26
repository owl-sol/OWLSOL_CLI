use crate::ui::formatter;
use anyhow::{Context, Result};
use colored::Colorize;
use owlsol_core::{CompressionAlgorithm, Compressor};
use std::fs;

pub async fn execute(input: String, verbose: bool) -> Result<()> {
    println!("{}", "🦉 OWLSOL Statistics".bright_cyan().bold());
    println!();

    let data = fs::read(&input).with_context(|| format!("Failed to read: {}", input))?;

    println!("  {} {}", "File:".bright_white(), input.bright_cyan());
    println!("  {} {} bytes", "Size:".bright_white(), data.len());
    println!();

    let compressor = Compressor::new();

    // Analyze data
    if verbose {
        let analysis = compressor.analyze_data(&data);
        formatter::print_data_analysis(&analysis);
        println!();
    }

    // Try all algorithms
    let algorithms = vec![
        CompressionAlgorithm::Huffman,
        CompressionAlgorithm::Dictionary,
        CompressionAlgorithm::RunLength,
        CompressionAlgorithm::Hybrid,
    ];

    println!("{}", "Compression Analysis:".bright_yellow().bold());
    println!();

    let mut best_algo = CompressionAlgorithm::None;
    let mut best_size = data.len();

    for algo in algorithms {
        if let Ok(result) = compressor.compress_with_algorithm(&data, Some(algo)) {
            formatter::print_algorithm_stats(&algo, &result.metadata);

            if result.data.len() < best_size {
                best_size = result.data.len();
                best_algo = algo;
            }
        }
    }

    println!();
    println!(
        "  {} {}",
        "Recommended:".bright_green().bold(),
        best_algo.as_str().bright_yellow()
    );
    println!(
        "  {} {} bytes ({:.2}% reduction)",
        "Best compression:".bright_green().bold(),
        best_size,
        (1.0 - best_size as f64 / data.len() as f64) * 100.0
    );

    Ok(())
}
