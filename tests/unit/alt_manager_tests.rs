#[cfg(test)]
mod alt_manager_unit_tests {
    use owlsol_cli::core::alt_manager::*;

    #[test]
    fn test_should_use_alt_true() {
        let amount = 1_000_000_000;
        let threshold = 500_000_000;
        assert!(should_use_alt(amount, threshold));
    }

    #[test]
    fn test_should_use_alt_false() {
        let amount = 100_000_000;
        let threshold = 500_000_000;
        assert!(!should_use_alt(amount, threshold));
    }

    #[test]
    fn test_calculate_alt_savings_basic() {
        let base_fee = 10000;
        let alt_fee = 8000;
        let savings = calculate_alt_savings(base_fee, alt_fee);
        assert_eq!(savings, 2000);
    }

    #[test]
    fn test_calculate_alt_savings_no_savings() {
        let base_fee = 10000;
        let alt_fee = 10000;
        let savings = calculate_alt_savings(base_fee, alt_fee);
        assert_eq!(savings, 0);
    }

    #[test]
    fn test_calculate_alt_savings_negative() {
        let base_fee = 8000;
        let alt_fee = 10000;
        let savings = calculate_alt_savings(base_fee, alt_fee);
        assert_eq!(savings, 0);
    }

    #[test]
    fn test_get_jupiter_alt_returns_string() {
        let alt = get_jupiter_alt();
        assert!(!alt.is_empty());
        assert!(alt.contains("ALT"));
    }
}
