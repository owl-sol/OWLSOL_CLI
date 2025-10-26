#[cfg(test)]
mod fee_optimizer_integration_tests {
    use owlsol_cli::core::fee_optimizer::*;
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::commitment_config::CommitmentConfig;
    use tokio;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_fee_optimizer_devnet() {
        let rpc_url = "https://api.devnet.solana.com";
        let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
        let fees = client.get_recent_prioritization_fees().unwrap_or_default();
        let fee_values: Vec<u64> = fees.iter().map(|f| f.prioritization_fee).collect();
        let strat = FeeStrategy::Percentile(80);
        let optimal_fee = get_optimal_priority_fee(&fee_values, &strat);
        assert!(optimal_fee > 0, "Optimal fee should be positive");
    }
}
