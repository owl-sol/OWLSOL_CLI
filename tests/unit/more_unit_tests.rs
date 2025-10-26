#[cfg(test)]
mod more_unit_tests {
    use owlsol_cli::core::jupiter::*;
    use owlsol_cli::core::fee_optimizer::*;
    use owlsol_cli::core::alt_manager::*;
    use owlsol_cli::core::transaction::*;
    use owlsol_cli::core::safety::*;

    #[test]
    fn test_token_mint_btc() {
        assert!(get_token_mint("BTC").is_err());
    }

    #[test]
    fn test_token_mint_eth() {
        assert!(get_token_mint("ETH").is_err());
    }

    #[test]
    fn test_fee_strategy_percentile_50() {
        let strat = FeeStrategy::Percentile(50);
        assert_eq!(strat, FeeStrategy::Percentile(50));
    }

    #[test]
    fn test_fee_strategy_buffer_0() {
        let strat = FeeStrategy::Buffer(0);
        assert_eq!(strat, FeeStrategy::Buffer(0));
    }

    #[test]
    fn test_should_use_alt_zero() {
        assert!(!should_use_alt(0, 100));
    }

    #[test]
    fn test_calculate_alt_savings_zero() {
        assert_eq!(calculate_alt_savings(0, 0), 0);
    }

    #[test]
    fn test_decode_and_optimize_transaction_invalid_fee() {
        let base64_tx = "AAECAwQFBgcICQoLDA0ODw==";
        let priority_fee = u64::MAX;
        let result = decode_and_optimize_transaction(base64_tx, priority_fee);
        assert!(result.is_ok());
    }

    #[test]
    fn test_perform_safety_checks_invalid_amount() {
        let result = perform_safety_checks("SOL", "USDC", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_swap_params_same_token() {
        let result = validate_swap_params("SOL", "SOL", 1_000_000);
        assert!(result.is_err());
    }
}
