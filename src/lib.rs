pub mod core;

// Re-export commonly used types
pub use core::{
    jupiter::{get_token_mint, ultra_swap_order},
    fee_optimizer::{get_optimal_priority_fee, FeeStrategy, FeeSavingsAnalysis},
    alt_manager::get_jupiter_alt,
    transaction::{send_optimized_transaction, simulate_transaction},
    safety::{perform_safety_checks, SafetyChecks},
};
