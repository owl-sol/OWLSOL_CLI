use anyhow::{Context, Result};
use solana_client::rpc_client::RpcClient;

/// Fee strategy options
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FeeStrategy {
    Economy,  // P50 - 50th percentile
    Standard, // P65 - 65th percentile
    Fast,     // P75 - 75th percentile
}

impl FeeStrategy {
    pub fn percentile(&self) -> usize {
        match self {
            FeeStrategy::Economy => 50,
            FeeStrategy::Standard => 65,
            FeeStrategy::Fast => 75,
        }
    }
}

/// Get optimal priority fee based on recent network activity
pub async fn get_optimal_priority_fee(rpc: &RpcClient, strategy: FeeStrategy) -> Result<u64> {
    // Fetch recent prioritization fees from last blocks
    let recent_fees = rpc
        .get_recent_prioritization_fees(&[])
        .context("Failed to fetch recent prioritization fees")?;

    if recent_fees.is_empty() {
        // Fallback to conservative default if no data
        return Ok(1000);
    }

    // Extract fee values
    let mut fees: Vec<u64> = recent_fees
        .into_iter()
        .map(|r| r.prioritization_fee)
        .filter(|&fee| fee > 0)
        .collect();

    if fees.is_empty() {
        return Ok(1000);
    }

    // Sort fees
    fees.sort_unstable();

    // Calculate percentile index
    let percentile = strategy.percentile();
    let index = (fees.len() * percentile) / 100;
    let index = index.min(fees.len() - 1);

    // Get base fee at percentile
    let base_fee = fees[index];

    // Add 10% safety buffer
    let optimal_fee = (base_fee as f64 * 1.1) as u64;

    // Ensure minimum fee of 1 micro-lamport
    Ok(optimal_fee.max(1))
}

/// Analyze potential savings vs typical user behavior
pub fn analyze_savings(optimal_fee: u64, estimated_cu: u32) -> FeeSavingsAnalysis {
    // Typical user behavior: request 200k CU with 50% higher priority fee
    let typical_cu = 200_000u64;
    let typical_fee_rate = (optimal_fee as f64 * 1.5) as u64;

    // Calculate costs
    let owlsol_cost = (estimated_cu as u64 * optimal_fee) as f64 / 1_000_000_000.0;
    let typical_cost = (typical_cu * typical_fee_rate) as f64 / 1_000_000_000.0;

    // Calculate savings
    let absolute_savings = typical_cost - owlsol_cost;
    let percentage_savings = if typical_cost > 0.0 {
        (absolute_savings / typical_cost) * 100.0
    } else {
        0.0
    };

    FeeSavingsAnalysis {
        owlsol_fee: owlsol_cost,
        typical_fee: typical_cost,
        absolute_savings,
        percentage_savings,
        optimal_fee_rate: optimal_fee,
        optimized_cu: estimated_cu,
    }
}

#[derive(Debug, Clone)]
pub struct FeeSavingsAnalysis {
    pub owlsol_fee: f64,
    pub typical_fee: f64,
    pub absolute_savings: f64,
    pub percentage_savings: f64,
    pub optimal_fee_rate: u64,
    pub optimized_cu: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_savings() {
        let optimal_fee = 5000; // 5000 micro-lamports per CU
        let estimated_cu = 150_000;

        let analysis = analyze_savings(optimal_fee, estimated_cu);

        // OWLSOL: 150k CU × 5000 = 750M micro-lamports = 0.00075 SOL
        assert!((analysis.owlsol_fee - 0.00075).abs() < 0.00001);

        // Typical: 200k CU × 7500 = 1500M micro-lamports = 0.0015 SOL
        assert!((analysis.typical_fee - 0.0015).abs() < 0.00001);

        // Savings: 50%
        assert!((analysis.percentage_savings - 50.0).abs() < 1.0);
    }
}
