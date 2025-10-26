mod commands;
mod ui;

use clap::{Parser, Subcommand};
use commands::{benchmark, compress, decompress, stats};

#[derive(Parser)]
#[command(name = "owlsol")]
#[command(about = "🦉 OWLSOL - Solana Account Storage Compression", long_about = None)]
#[command(version)]
#[command(author = "OWLSOL Team")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress data and optionally deploy to Solana
    Compress {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Output file path (optional, defaults to <input>.owlsol)
        #[arg(short, long)]
        output: Option<String>,

        /// Compression algorithm (huffman, dictionary, rle, hybrid, auto)
        #[arg(short, long, default_value = "auto")]
        algorithm: String,

        /// Deploy compressed data to Solana devnet
        #[arg(short, long)]
        deploy: bool,

        /// Show results in ratatui UI
        #[arg(long)]
        ui: bool,
    },

    /// Decompress data from file or Solana account
    Decompress {
        /// Input file path or Solana account address
        #[arg(short, long)]
        input: String,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,

        /// Fetch from Solana account instead of file
        #[arg(short, long)]
        from_solana: bool,
    },

    /// Show compression statistics for a file
    Stats {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Show detailed analysis
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Run compression benchmarks
    Benchmark {
        /// Input file or directory
        #[arg(short, long)]
        input: String,

        /// Number of iterations per test
        #[arg(short = 'n', long, default_value = "100")]
        iterations: usize,

        /// Output report file (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compress {
            input,
            output,
            algorithm,
            deploy,
            ui,
        } => {
            let result =
                compress::execute(input.clone(), output.clone(), algorithm.clone(), deploy).await?;
            if ui {
                use ui::ratatui_ui::show_compression_stats;
                let stats = vec![
                    (
                        "Algorithm".to_string(),
                        result.metadata.algorithm.as_str().to_string(),
                    ),
                    (
                        "Original size".to_string(),
                        result.metadata.original_size.to_string(),
                    ),
                    (
                        "Compressed size".to_string(),
                        result.metadata.compressed_size.to_string(),
                    ),
                    (
                        "Compression ratio".to_string(),
                        format!("{:.2}%", result.metadata.compression_percentage()),
                    ),
                    (
                        "Space saved".to_string(),
                        result.metadata.space_saved().to_string(),
                    ),
                    (
                        "Checksum".to_string(),
                        format!("{:08x}", result.metadata.checksum),
                    ),
                ];
                show_compression_stats(&stats)?;
            }
        }
        Commands::Decompress {
            input,
            output,
            from_solana,
        } => {
            decompress::execute(input, output, from_solana).await?;
        }
        Commands::Stats { input, verbose } => {
            stats::execute(input, verbose).await?;
        }
        Commands::Benchmark {
            input,
            iterations,
            output,
        } => {
            benchmark::execute(input, iterations, output).await?;
        }
    }

    Ok(())
}
