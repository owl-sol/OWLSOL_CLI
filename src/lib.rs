pub mod core;

// Re-export commonly used types
pub use core::{
    jupiter::{get_quote, get_swap_transaction, get_token_mint, QuoteResponse, SwapResponse},
    fee_optimizer::{get_optimal_priority_fee, FeeStrategy, FeeSavingsAnalysis},
    alt_manager::get_jupiter_alt,
    transaction::{send_optimized_transaction, simulate_transaction},
    safety::{perform_safety_checks, SafetyChecks},
};
