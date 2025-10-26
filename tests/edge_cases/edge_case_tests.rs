#[cfg(test)]
mod edge_case_tests {
    use owlsol_cli::core::jupiter::*;
    use owlsol_cli::core::fee_optimizer::*;
    use owlsol_cli::core::alt_manager::*;
    use owlsol_cli::core::transaction::*;
    use owlsol_cli::core::safety::*;

    #[test]
    fn test_extremely_large_amount() {
        let result = validate_swap_params("SOL", "USDC", u64::MAX);
        assert!(result.is_err() || result.is_ok()); // Should not panic
    }

    #[test]
    fn test_negative_amount() {
        let result = validate_swap_params("SOL", "USDC", 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_token_symbols() {
        let result = get_token_mint("???");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_token_symbol() {
        let result = get_token_mint("");
        assert!(result.is_err());
    }

    #[test]
    fn test_fee_optimizer_empty_fees() {
        let fees: Vec<u64> = vec![];
        let strat = FeeStrategy::Percentile(80);
        let optimal = get_optimal_priority_fee(&fees, &strat);
        assert_eq!(optimal, 0);
    }

    #[test]
    fn test_alt_manager_empty_alt() {
        let alt = get_jupiter_alt();
        assert!(!alt.is_empty());
    }

    #[test]
    fn test_transaction_invalid_base64() {
        let result = decode_and_optimize_transaction("not_base64", 5000);
        assert!(result.is_err());
    }
}
