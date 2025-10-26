use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use owlsol_core::{CompressionAlgorithm, Compressor};
use std::fs;
use std::time::{Duration, Instant};

pub async fn execute(input: String, iterations: usize, output: Option<String>) -> Result<()> {
    println!("{}", "🦉 OWLSOL Benchmark".bright_cyan().bold());
    println!();

    let data = fs::read(&input)?;
    println!("  {} {}", "File:".bright_white(), input.bright_cyan());
    println!("  {} {} bytes", "Size:".bright_white(), data.len());
    println!("  {} {}", "Iterations:".bright_white(), iterations);
    println!();

    let compressor = Compressor::new();

    // Benchmark each algorithm
    let algorithms = vec![
        ("Huffman", CompressionAlgorithm::Huffman),
        ("Dictionary", CompressionAlgorithm::Dictionary),
        ("RLE", CompressionAlgorithm::RunLength),
        ("Hybrid", CompressionAlgorithm::Hybrid),
    ];

    println!("{}", "Running benchmarks...".bright_yellow().bold());
    println!();

    let mut results = Vec::new();

    for (name, algo) in algorithms {
        let pb = ProgressBar::new(iterations as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "  {{spinner:.cyan}} {:<12} [{{bar:40.cyan/blue}}] {{pos}}/{{len}}",
                    name
                ))
                .unwrap()
                .progress_chars("█▓░"),
        );

        let mut total_compress_time = Duration::ZERO;
        let mut total_decompress_time = Duration::ZERO;
        let mut compressed_size = 0;

        for _ in 0..iterations {
            // Compression benchmark
            let start = Instant::now();
            let result = compressor.compress_with_algorithm(&data, Some(algo))?;
            total_compress_time += start.elapsed();
            compressed_size = result.data.len();

            // Decompression benchmark
            let decompressor = owlsol_core::Decompressor::new();
            let start = Instant::now();
            let _ = decompressor.decompress(&result.data, &result.metadata)?;
            total_decompress_time += start.elapsed();

            pb.inc(1);
        }

        pb.finish_with_message("Done");

        let avg_compress = total_compress_time.as_micros() as f64 / iterations as f64;
        let avg_decompress = total_decompress_time.as_micros() as f64 / iterations as f64;
        let ratio = compressed_size as f64 / data.len() as f64;

        results.push((name, avg_compress, avg_decompress, compressed_size, ratio));
    }

    // Display results
    println!();
    println!("{}", "Benchmark Results:".bright_green().bold());
    println!();
    println!(
        "  {:<12} {:<15} {:<15} {:<15} {:<10}",
        "Algorithm",
        "Compress (μs)",
        "Decompress (μs)",
        "Size (bytes)",
        "Ratio"
    );
    println!("  {}", "─".repeat(70).bright_black());

    for (name, comp_time, decomp_time, size, ratio) in &results {
        println!(
            "  {:<12} {:>14.2} {:>14.2} {:>14} {:>9.2}%",
            name.bright_yellow(),
            comp_time,
            decomp_time,
            size,
            ratio * 100.0
        );
    }

    println!();

    // Calculate throughput
    let best_compress = results
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
    let best_ratio = results
        .iter()
        .min_by(|a, b| a.4.partial_cmp(&b.4).unwrap())
        .unwrap();

    println!("  {} {}", "Fastest compression:".bright_green(), best_compress.0);
    println!("  {} {}", "Best ratio:".bright_green(), best_ratio.0);

    // Calculate throughput
    let throughput_mb_s = (data.len() as f64 / 1024.0 / 1024.0)
        / (best_compress.1 / 1_000_000.0);

    println!();
    println!(
        "  {} {:.2} MB/s",
        "Peak throughput:".bright_cyan(),
        throughput_mb_s
    );

    // Save report
    if let Some(output_path) = output {
        let report = format!(
            "OWLSOL Benchmark Report\n\
             =======================\n\
             File: {}\n\
             Size: {} bytes\n\
             Iterations: {}\n\
             Results:\n\
             {}\n",
            input,
            data.len(),
            iterations,
            results
                .iter()
                .map(|(name, comp, decomp, size, ratio)| {
                    format!(
                        "{}: compress={:.2}μs, decompress={:.2}μs, size={}, ratio={:.2}%",
                        name,
                        comp,
                        decomp,
                        size,
                        ratio * 100.0
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        );

        fs::write(&output_path, report)?;
        println!();
        println!("  ✓ Report saved to {}", output_path.bright_cyan());
    }

    Ok(())
}
