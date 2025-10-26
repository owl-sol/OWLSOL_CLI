#[cfg(test)]
mod fee_optimizer_unit_tests {
    use owlsol_cli::core::fee_optimizer::*;

    #[test]
    fn test_fee_strategy_percentile() {
        let strat = FeeStrategy::Percentile(90);
        assert_eq!(strat, FeeStrategy::Percentile(90));
    }

    #[test]
    fn test_fee_strategy_buffer() {
        let strat = FeeStrategy::Buffer(5000);
        assert_eq!(strat, FeeStrategy::Buffer(5000));
    }

    #[test]
    fn test_fee_strategy_default() {
        let strat = FeeStrategy::default();
        assert_eq!(strat, FeeStrategy::Percentile(80));
    }

    #[test]
    fn test_analyze_savings_basic() {
        let base_fee = 10000;
        let optimized_fee = 8000;
        let savings = analyze_savings(base_fee, optimized_fee);
        assert_eq!(savings, 2000);
    }

    #[test]
    fn test_analyze_savings_no_savings() {
        let base_fee = 10000;
        let optimized_fee = 10000;
        let savings = analyze_savings(base_fee, optimized_fee);
        assert_eq!(savings, 0);
    }

    #[test]
    fn test_analyze_savings_negative() {
        let base_fee = 8000;
        let optimized_fee = 10000;
        let savings = analyze_savings(base_fee, optimized_fee);
        assert_eq!(savings, 0);
    }

    #[test]
    fn test_get_optimal_priority_fee_percentile() {
        let fees = vec![1000, 2000, 3000, 4000, 5000];
        let strat = FeeStrategy::Percentile(80);
        let optimal = get_optimal_priority_fee(&fees, &strat);
        assert_eq!(optimal, 5000);
    }

    #[test]
    fn test_get_optimal_priority_fee_buffer() {
        let fees = vec![1000, 2000, 3000, 4000, 5000];
        let strat = FeeStrategy::Buffer(500);
        let optimal = get_optimal_priority_fee(&fees, &strat);
        assert_eq!(optimal, 5500);
    }

    #[test]
    fn test_get_optimal_priority_fee_empty_fees() {
        let fees: Vec<u64> = vec![];
        let strat = FeeStrategy::Percentile(80);
        let optimal = get_optimal_priority_fee(&fees, &strat);
        assert_eq!(optimal, 0);
    }

    #[test]
    fn test_get_optimal_priority_fee_single_fee() {
        let fees = vec![12345];
        let strat = FeeStrategy::Percentile(80);
        let optimal = get_optimal_priority_fee(&fees, &strat);
        assert_eq!(optimal, 12345);
    }
}
