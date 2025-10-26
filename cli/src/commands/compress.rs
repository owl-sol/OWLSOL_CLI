use crate::ui::formatter;
use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use owlsol_core::{CompressionAlgorithm, Compressor};
use owlsol_solana::SolanaClient;
use std::fs;

// ...existing code...
pub async fn execute(
    input: String,
    output: Option<String>,
    algorithm: String,
    deploy: bool,
) -> Result<owlsol_core::CompressionResult> {
    println!("{}", "🦉 OWLSOL Compression".bright_cyan().bold());
    println!();

    // Read input file
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Reading input file...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let data = fs::read(&input).with_context(|| format!("Failed to read file: {}", input))?;

    spinner.finish_with_message(format!("✓ Read {} bytes", data.len()));

    // Parse algorithm
    let algo = parse_algorithm(&algorithm);

    // Compress
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Compressing...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let start = std::time::Instant::now();
    let compressor = Compressor::new();
    let result = compressor
        .compress_with_algorithm(&data, algo)
        .context("Compression failed")?;
    let elapsed = start.elapsed();

    spinner.finish_with_message(format!("✓ Compressed in {:.2}ms", elapsed.as_millis()));

    // Display results
    println!();
    formatter::print_compression_result(&result, elapsed);

    // Save to file
    let output_path = output.unwrap_or_else(|| format!("{}.owlsol", input));

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Saving compressed data...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    // Save compressed data
    fs::write(&output_path, &result.data)
        .with_context(|| format!("Failed to write to: {}", output_path))?;

    // Save metadata
    let metadata_path = format!("{}.meta.json", output_path);
    let metadata_json = serde_json::to_string_pretty(&result.metadata)?;
    fs::write(&metadata_path, metadata_json)?;

    spinner.finish_with_message(format!("✓ Saved to {}", output_path.bright_cyan()));
    println!("  ✓ Saved to {}", output_path.bright_cyan());
    println!("  Metadata: {}", metadata_path.bright_black());

    // Deploy to Solana
    if deploy {
        println!();
        deploy_to_solana(&result).await?;
    }

    Ok(result)
}

fn parse_algorithm(algo: &str) -> Option<CompressionAlgorithm> {
    match algo.to_lowercase().as_str() {
        "huffman" => Some(CompressionAlgorithm::Huffman),
        "dictionary" | "dict" => Some(CompressionAlgorithm::Dictionary),
        "rle" | "runlength" => Some(CompressionAlgorithm::RunLength),
        "hybrid" => Some(CompressionAlgorithm::Hybrid),
        "none" => Some(CompressionAlgorithm::None),
        "auto" => None,
        _ => None,
    }
}

async fn deploy_to_solana(result: &owlsol_core::CompressionResult) -> Result<()> {
    println!(
        "{}",
        "📡 Deploying to Solana Devnet...".bright_yellow().bold()
    );

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Connecting to devnet...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let client = SolanaClient::devnet().context("Failed to connect to Solana devnet")?;

    spinner.finish_with_message("✓ Connected");

    // Check balance
    let balance = client.get_balance()?;
    let balance_sol = balance as f64 / 1_000_000_000.0;

    println!();
    println!("  {} {}", "Wallet:".bright_white(), client.pubkey());
    println!("  {} {:.4} SOL", "Balance:".bright_white(), balance_sol);

    // Request airdrop if needed
    if balance < 100_000_000 {
        let spinner = ProgressBar::new_spinner();
        spinner.set_message("Requesting airdrop...");
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));

        client.airdrop(1_000_000_000)?;

        spinner.finish_with_message("✓ Received 1 SOL airdrop");
    }

    // Calculate rent
    let rent = client.calculate_rent(result.data.len())?;
    let rent_sol = rent as f64 / 1_000_000_000.0;

    println!();
    println!(
        "  {} {} bytes",
        "Compressed size:".bright_white(),
        result.data.len()
    );
    println!("  {} {:.6} SOL", "Storage rent:".bright_white(), rent_sol);

    // Calculate savings
    let original_rent = client.calculate_rent(result.metadata.original_size as usize)?;
    let original_rent_sol = original_rent as f64 / 1_000_000_000.0;
    let savings_sol = original_rent_sol - rent_sol;

    println!(
        "  {} {:.6} SOL",
        "Original would cost:".bright_black(),
        original_rent_sol
    );
    println!("  {} {:.6} SOL saved!", "💰".bright_green(), savings_sol);

    println!();
    println!(
        "  {}",
        "Note: Actual on-chain deployment requires the Solana program to be deployed."
            .bright_black()
            .italic()
    );
    println!(
        "  {}",
        "This is a simulation showing potential savings."
            .bright_black()
            .italic()
    );

    Ok(())
}
