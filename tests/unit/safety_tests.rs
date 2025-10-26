#[cfg(test)]
mod safety_unit_tests {
    use owlsol_cli::core::safety::*;

    #[test]
    fn test_perform_safety_checks_valid() {
        let params = ("SOL", "USDC", 1_000_000);
        let result = perform_safety_checks(params.0, params.1, params.2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_perform_safety_checks_invalid_token() {
        let params = ("INVALID", "USDC", 1_000_000);
        let result = perform_safety_checks(params.0, params.1, params.2);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Unsupported token"));
    }

    #[test]
    fn test_perform_safety_checks_zero_amount() {
        let params = ("SOL", "USDC", 0);
        let result = perform_safety_checks(params.0, params.1, params.2);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Amount must be positive"));
    }

    #[test]
    fn test_validate_swap_params_valid() {
        let params = ("SOL", "USDC", 1_000_000);
        let result = validate_swap_params(params.0, params.1, params.2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_swap_params_invalid_token() {
        let params = ("INVALID", "USDC", 1_000_000);
        let result = validate_swap_params(params.0, params.1, params.2);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_swap_params_zero_amount() {
        let params = ("SOL", "USDC", 0);
        let result = validate_swap_params(params.0, params.1, params.2);
        assert!(result.is_err());
    }
}
