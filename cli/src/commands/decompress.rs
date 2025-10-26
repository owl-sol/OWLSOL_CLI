use crate::ui::formatter;
use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use owlsol_core::{CompressionMetadata, Decompressor};
use std::fs;

pub async fn execute(input: String, output: Option<String>, from_solana: bool) -> Result<()> {
    println!(
        "{}",
        "🦉 OWLSOL Decompression".bright_cyan().bold()
    );
    println!();

    if from_solana {
        println!(
            "  {}",
            "Note: Fetching from Solana requires deployed program."
                .bright_yellow()
                .italic()
        );
        println!(
            "  {}",
            "Coming soon in future versions!".bright_yellow().italic()
        );
        return Ok(());
    }

    // Read compressed file
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Reading compressed file...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let data = fs::read(&input).with_context(|| format!("Failed to read: {}", input))?;

    // Read metadata
    let metadata_path = format!("{}.meta.json", input);
    let metadata_json = fs::read_to_string(&metadata_path)
        .with_context(|| format!("Failed to read metadata: {}", metadata_path))?;
    let metadata: CompressionMetadata = serde_json::from_str(&metadata_json)?;

    spinner.finish_with_message(format!(
        "✓ Read {} bytes (compressed)",
        data.len()
    ));

    // Decompress
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Decompressing...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let start = std::time::Instant::now();
    let decompressor = Decompressor::new();
    let decompressed = decompressor
        .decompress(&data, &metadata)
        .context("Decompression failed")?;
    let elapsed = start.elapsed();

    spinner.finish_with_message(format!("✓ Decompressed in {:.2}ms", elapsed.as_millis()));

    // Verify size
    if decompressed.len() != metadata.original_size as usize {
        anyhow::bail!(
            "Size mismatch! Expected {}, got {}",
            metadata.original_size,
            decompressed.len()
        );
    }

    println!();
    formatter::print_decompression_result(&metadata, decompressed.len(), elapsed);

    // Save output
    if let Some(output_path) = output {
        fs::write(&output_path, &decompressed)
            .with_context(|| format!("Failed to write: {}", output_path))?;
    println!();
    println!("  ✓ Saved to {}", output_path.bright_cyan());
    let metadata_path = format!("{}.meta.json", input);
    println!("  Metadata: {}", metadata_path.bright_black());
    }

    Ok(())
}
