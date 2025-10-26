use colored::Colorize;
use owlsol_core::{analyzer::DataAnalysis, CompressionAlgorithm, CompressionMetadata, CompressionResult};
use std::time::Duration;

pub fn print_compression_result(result: &CompressionResult, elapsed: Duration) {
    let meta = &result.metadata;

    println!("{}", "  Compression Results:".bright_green().bold());
    println!(
        "    {} {}",
        "Algorithm:".bright_white(),
        meta.algorithm.as_str().bright_yellow()
    );
    println!(
        "    {} {} bytes",
        "Original size:".bright_white(),
        meta.original_size
    );
    println!(
        "    {} {} bytes",
        "Compressed size:".bright_white(),
        meta.compressed_size
    );
    println!(
        "    {} {}",
        "Compression ratio:".bright_white(),
        format!("{:.2}%", meta.compression_percentage())
            .bright_green()
            .bold()
    );
    println!(
        "    {} {} bytes",
        "Space saved:".bright_white(),
        meta.space_saved()
    );
    println!(
        "    {} {:.2}ms",
        "Time taken:".bright_white(),
        elapsed.as_millis()
    );

    if meta.checksum != 0 {
        println!(
            "    {} {:08x}",
            "Checksum:".bright_black(),
            meta.checksum
        );
    }
}

pub fn print_decompression_result(
    meta: &CompressionMetadata,
    decompressed_size: usize,
    elapsed: Duration,
) {
    println!("{}", "  Decompression Results:".bright_green().bold());
    println!(
        "    {} {}",
        "Algorithm:".bright_white(),
        meta.algorithm.as_str().bright_yellow()
    );
    println!(
        "    {} {} bytes",
        "Decompressed size:".bright_white(),
        decompressed_size
    );
    println!(
        "    {} {:.2}ms",
        "Time taken:".bright_white(),
        elapsed.as_millis()
    );
    println!("    {} ✓", "Integrity:".bright_white());
}

pub fn print_algorithm_stats(algo: &CompressionAlgorithm, meta: &CompressionMetadata) {
    println!(
        "  {} {}",
        "▸".bright_blue(),
        algo.as_str().bright_yellow()
    );
    println!(
        "    Compressed: {} bytes",
        meta.compressed_size
    );
    println!("    Ratio: {:.2}%", meta.compression_percentage());
    println!("    Savings: {} bytes", meta.space_saved());
    println!();
}

pub fn print_data_analysis(analysis: &DataAnalysis) {
    println!("{}", "  Data Analysis:".bright_yellow().bold());
    println!(
        "    {} {:.2}",
        "Entropy:".bright_white(),
        analysis.entropy
    );
    println!(
        "    {} {}",
        "Unique bytes:".bright_white(),
        analysis.unique_bytes
    );
    println!(
        "    {} {}",
        "Max run length:".bright_white(),
        analysis.max_run_length
    );
    println!(
        "    {} {:.2}",
        "Avg run length:".bright_white(),
        analysis.avg_run_length
    );
    println!(
        "    {} {}",
        "Repeated sequences:".bright_white(),
        analysis.repeated_sequences
    );

    println!();
    println!("    {} Characteristics:", "Characteristics:".bright_white());
    
    if analysis.is_random() {
        println!("      • {} Random/encrypted data", "⚠".bright_yellow());
    }
    if analysis.has_runs() {
        println!("      • {} Contains long runs", "✓".bright_green());
    }
    if analysis.has_patterns() {
        println!("      • {} Has repeated patterns", "✓".bright_green());
    }
    if analysis.is_text_like() {
        println!("      • {} Text-like data", "✓".bright_green());
    }
}
