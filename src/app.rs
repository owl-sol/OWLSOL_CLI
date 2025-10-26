use anyhow::{Context, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
use std::time::{Duration, Instant};

use crate::core::{fee_optimizer, jupiter, transaction};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeeSpeed {
    Economy,
    Standard,
    Fast,
}

impl FeeSpeed {
    pub fn to_string(&self) -> &'static str {
        match self {
            FeeSpeed::Economy => "Economy",
            FeeSpeed::Standard => "Standard",
            FeeSpeed::Fast => "Fast",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputField {
    FromToken,
    ToToken,
    Amount,
    Speed,
}

pub struct App {
    // User state
    pub wallet: Keypair,
    pub wallet_pubkey: String,
    pub balance: f64,

    // Input fields
    pub from_token: String,
    pub to_token: String,
    pub amount: String,
    pub speed: FeeSpeed,
    pub active_field: InputField,

    // Optimization data
    pub optimal_fee: u64,
    pub estimated_cu: u32,
    pub using_alt: bool,
    pub routes_analyzed: usize,
    pub estimated_output: f64,

    // Cost comparison
    pub owlsol_fee: f64,
    pub normal_fee: f64,
    pub savings_pct: f64,

    // Status
    pub status: String,
    pub is_ready: bool,
    pub is_loading: bool,
    pub error: Option<String>,

    // Backend
    pub rpc: RpcClient,
    pub network: String,
    pub last_refresh: Instant,
    pub last_signature: Option<String>,
}

impl App {
    pub async fn new() -> Result<Self> {
        // Load wallet from standard Solana CLI location
        let wallet_path = shellexpand::tilde("~/.config/solana/id.json");
        let wallet = solana_sdk::signature::read_keypair_file(wallet_path.as_ref())
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .context("Failed to load wallet. Make sure you have a Solana wallet at ~/.config/solana/id.json")?;

        let wallet_pubkey = wallet.pubkey().to_string();

        // Setup RPC (default to mainnet, can be changed)
        let network = "mainnet".to_string();
        let rpc_url = Self::get_rpc_url(&network);
        let rpc = RpcClient::new(rpc_url);

        // Get balance
        let balance = match rpc.get_balance(&wallet.pubkey()) {
            Ok(lamports) => lamports as f64 / 1_000_000_000.0,
            Err(_) => 0.0, // If RPC fails, show 0 but don't crash
        };

        Ok(Self {
            wallet,
            wallet_pubkey,
            balance,

            // Default values
            from_token: "USDC".to_string(),
            to_token: "SOL".to_string(),
            amount: "100".to_string(),
            speed: FeeSpeed::Standard,
            active_field: InputField::Amount,

            // Optimization data (will be filled on refresh)
            optimal_fee: 0,
            estimated_cu: 150_000,
            using_alt: true,
            routes_analyzed: 0,
            estimated_output: 0.0,

            // Costs
            owlsol_fee: 0.0,
            normal_fee: 0.0,
            savings_pct: 0.0,

            // Status
            status: "Initializing...".to_string(),
            is_ready: false,
            is_loading: false,
            error: None,

            // Backend
            rpc,
            network,
            last_refresh: Instant::now() - Duration::from_secs(100), // Force initial refresh
            last_signature: None,
        })
    }

    fn get_rpc_url(network: &str) -> String {
        match network {
            "devnet" => "https://api.devnet.solana.com".to_string(),
            "mainnet" => "https://api.mainnet-beta.solana.com".to_string(),
            _ => "https://api.mainnet-beta.solana.com".to_string(),
        }
    }

    // Navigation
    pub fn next_field(&mut self) {
        self.active_field = match self.active_field {
            InputField::FromToken => InputField::ToToken,
            InputField::ToToken => InputField::Amount,
            InputField::Amount => InputField::Speed,
            InputField::Speed => InputField::FromToken,
        };
    }

    pub fn previous_field(&mut self) {
        self.active_field = match self.active_field {
            InputField::FromToken => InputField::Speed,
            InputField::ToToken => InputField::FromToken,
            InputField::Amount => InputField::ToToken,
            InputField::Speed => InputField::Amount,
        };
    }

    pub fn next_option(&mut self) {
        if self.active_field == InputField::Speed {
            self.speed = match self.speed {
                FeeSpeed::Economy => FeeSpeed::Standard,
                FeeSpeed::Standard => FeeSpeed::Fast,
                FeeSpeed::Fast => FeeSpeed::Economy,
            };
        } else if self.active_field == InputField::FromToken {
            self.from_token = self.get_next_token(&self.from_token);
        } else if self.active_field == InputField::ToToken {
            self.to_token = self.get_next_token(&self.to_token);
        }
    }

    pub fn previous_option(&mut self) {
        if self.active_field == InputField::Speed {
            self.speed = match self.speed {
                FeeSpeed::Fast => FeeSpeed::Standard,
                FeeSpeed::Standard => FeeSpeed::Economy,
                FeeSpeed::Economy => FeeSpeed::Fast,
            };
        } else if self.active_field == InputField::FromToken {
            self.from_token = self.get_previous_token(&self.from_token);
        } else if self.active_field == InputField::ToToken {
            self.to_token = self.get_previous_token(&self.to_token);
        }
    }

    fn get_next_token(&self, current: &str) -> String {
        match current {
            "SOL" => "USDC".to_string(),
            "USDC" => "USDT".to_string(),
            "USDT" => "SOL".to_string(),
            _ => "SOL".to_string(),
        }
    }

    fn get_previous_token(&self, current: &str) -> String {
        match current {
            "SOL" => "USDT".to_string(),
            "USDC" => "SOL".to_string(),
            "USDT" => "USDC".to_string(),
            _ => "SOL".to_string(),
        }
    }

    // Input handling
    pub fn handle_input(&mut self, c: char) {
        match self.active_field {
            InputField::Amount => {
                if c.is_numeric() || c == '.' {
                    // Only allow one decimal point
                    if c == '.' && self.amount.contains('.') {
                        return;
                    }
                    self.amount.push(c);
                }
            }
            _ => {}
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.active_field == InputField::Amount {
            self.amount.pop();
        }
    }

    // Data refresh
    pub async fn refresh_data(&mut self) -> Result<()> {
        self.is_loading = true;
        self.error = None;

        // Step 1: Get optimal priority fee
        let fee_strategy = match self.speed {
            FeeSpeed::Economy => fee_optimizer::FeeStrategy::Economy,
            FeeSpeed::Standard => fee_optimizer::FeeStrategy::Standard,
            FeeSpeed::Fast => fee_optimizer::FeeStrategy::Fast,
        };

        match fee_optimizer::get_optimal_priority_fee(&self.rpc, fee_strategy).await {
            Ok(fee) => {
                self.optimal_fee = fee;
            }
            Err(e) => {
                self.error = Some(format!("Failed to fetch priority fees: {}", e));
                self.is_loading = false;
                self.is_ready = false;
                return Ok(());
            }
        }

        // Step 2: Parse amount
        let amount_parsed: f64 = match self.amount.parse() {
            Ok(amt) => amt,
            Err(_) => {
                self.error = Some("Invalid amount".to_string());
                self.is_loading = false;
                self.is_ready = false;
                return Ok(());
            }
        };

        if amount_parsed <= 0.0 {
            self.error = Some("Amount must be greater than 0".to_string());
            self.is_loading = false;
            self.is_ready = false;
            return Ok(());
        }

        // Step 3: Check if same token
        if self.from_token == self.to_token {
            self.error = Some("Cannot swap same token".to_string());
            self.is_loading = false;
            self.is_ready = false;
            return Ok(());
        }

        // Step 4: Get token mints
        let from_mint = match jupiter::get_token_mint(&self.from_token) {
            Ok(mint) => mint,
            Err(e) => {
                self.error = Some(format!("Invalid FROM token: {}", e));
                self.is_loading = false;
                self.is_ready = false;
                return Ok(());
            }
        };

        let to_mint = match jupiter::get_token_mint(&self.to_token) {
            Ok(mint) => mint,
            Err(e) => {
                self.error = Some(format!("Invalid TO token: {}", e));
                self.is_loading = false;
                self.is_ready = false;
                return Ok(());
            }
        };

        // Step 5: Convert amount to base units (assume 6 decimals)
        let amount_lamports = (amount_parsed * 1_000_000.0) as u64;

        // Step 6: Get Jupiter quote
        match jupiter::get_quote(&from_mint, &to_mint, amount_lamports).await {
            Ok(quote) => {
                // Update output
                self.estimated_output = quote.out_amount as f64 / 1_000_000.0;
                self.routes_analyzed = quote.routes_count.unwrap_or(1);

                // Estimate compute units (conservative)
                self.estimated_cu = 150_000;

                // Calculate fees
                // OWLSOL: Optimized CU (150k) with optimal fee
                self.owlsol_fee = (self.estimated_cu as f64 * self.optimal_fee as f64)
                    / 1_000_000_000.0;

                // Normal wallet: 200k CU with 50% higher fee (common overpayment)
                let normal_cu = 200_000;
                let normal_fee_rate = (self.optimal_fee as f64 * 1.5) as u64;
                self.normal_fee = (normal_cu as f64 * normal_fee_rate as f64) / 1_000_000_000.0;

                // Calculate savings
                if self.normal_fee > 0.0 {
                    self.savings_pct =
                        ((self.normal_fee - self.owlsol_fee) / self.normal_fee) * 100.0;
                } else {
                    self.savings_pct = 0.0;
                }

                // Ready to execute
                self.is_ready = true;
                self.status = "Ready - All checks passed ✓".to_string();
            }
            Err(e) => {
                self.error = Some(format!("Failed to get quote: {}", e));
                self.is_ready = false;
            }
        }

        self.is_loading = false;
        self.last_refresh = Instant::now();
        Ok(())
    }

    pub fn should_refresh(&self) -> bool {
        // Auto-refresh every 10 seconds
        self.last_refresh.elapsed() > Duration::from_secs(10)
    }

    pub fn can_execute(&self) -> bool {
        self.is_ready && !self.is_loading && self.error.is_none()
    }

    // Execute swap
    pub async fn execute_swap(&mut self) -> Result<()> {
        self.status = "Building transaction...".to_string();
        self.is_loading = true;

        // Get token mints
        let from_mint = jupiter::get_token_mint(&self.from_token)?;
        let to_mint = jupiter::get_token_mint(&self.to_token)?;
        let amount_parsed: f64 = self.amount.parse()?;
        let amount_lamports = (amount_parsed * 1_000_000.0) as u64;

        // Get quote again (fresh)
        self.status = "Getting fresh quote...".to_string();
        let quote = jupiter::get_quote(&from_mint, &to_mint, amount_lamports).await?;

        // Get swap transaction
        self.status = "Getting swap instructions...".to_string();
        let swap_response = jupiter::get_swap_transaction(&quote, &self.wallet.pubkey()).await?;

        // Build and send transaction
        self.status = "Sending transaction...".to_string();
        let signature = transaction::send_optimized_transaction(
            &self.rpc,
            &self.wallet,
            &swap_response.swap_transaction,
            self.optimal_fee,
        )
        .await?;

        // Success!
        self.last_signature = Some(signature.clone());
        self.status = format!("✅ Swap completed! Signature: {}", &signature[..8]);
        self.is_loading = false;
        self.is_ready = false;

        // Update balance
        if let Ok(lamports) = self.rpc.get_balance(&self.wallet.pubkey()) {
            self.balance = lamports as f64 / 1_000_000_000.0;
        }

        Ok(())
    }
}
