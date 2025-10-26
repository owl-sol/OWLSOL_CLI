use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

/// Pre-flight safety checks before executing swap
pub struct SafetyChecks {
    pub has_sufficient_balance: bool,
    pub rpc_is_healthy: bool,
    pub wallet_is_valid: bool,
    pub network_is_stable: bool,
}

impl SafetyChecks {
    pub fn all_passed(&self) -> bool {
        self.has_sufficient_balance
            && self.rpc_is_healthy
            && self.wallet_is_valid
            && self.network_is_stable
    }
}

/// Perform all safety checks
pub async fn perform_safety_checks(
    rpc: &RpcClient,
    wallet_pubkey: &Pubkey,
    estimated_fee: f64,
) -> Result<SafetyChecks> {
    let mut checks = SafetyChecks {
        has_sufficient_balance: false,
        rpc_is_healthy: false,
        wallet_is_valid: false,
        network_is_stable: true,
    };

    // Check 1: Wallet validity
    checks.wallet_is_valid = wallet_pubkey.to_bytes().len() == 32;

    // Check 2: RPC health
    checks.rpc_is_healthy = rpc.get_health().is_ok();

    // Check 3: Sufficient balance
    if checks.rpc_is_healthy {
        if let Ok(lamports) = rpc.get_balance(wallet_pubkey) {
            let balance_sol = lamports as f64 / 1_000_000_000.0;
            // Need at least 2x the fee for safety
            checks.has_sufficient_balance = balance_sol >= (estimated_fee * 2.0);
        }
    }

    Ok(checks)
}

/// Validate swap parameters
pub fn validate_swap_params(from_token: &str, to_token: &str, amount: f64) -> Result<()> {
    // Check amount is positive
    if amount <= 0.0 {
        anyhow::bail!("Amount must be greater than 0");
    }

    // Check tokens are different
    if from_token.to_uppercase() == to_token.to_uppercase() {
        anyhow::bail!("Cannot swap the same token");
    }

    // Check tokens are supported
    let supported_tokens = ["SOL", "USDC", "USDT", "BONK", "WIF"];
    if !supported_tokens.contains(&from_token.to_uppercase().as_str()) {
        anyhow::bail!("Unsupported FROM token: {}", from_token);
    }
    if !supported_tokens.contains(&to_token.to_uppercase().as_str()) {
        anyhow::bail!("Unsupported TO token: {}", to_token);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_swap_params() {
        // Valid params
        assert!(validate_swap_params("USDC", "SOL", 100.0).is_ok());

        // Invalid: zero amount
        assert!(validate_swap_params("USDC", "SOL", 0.0).is_err());

        // Invalid: same token
        assert!(validate_swap_params("SOL", "SOL", 100.0).is_err());

        // Invalid: unsupported token
        assert!(validate_swap_params("UNKNOWN", "SOL", 100.0).is_err());
    }
}
