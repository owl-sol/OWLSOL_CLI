#[cfg(test)]
mod tests {
    use owlsol_cli::core::{fee_optimizer, jupiter};
    use solana_client::rpc_client::RpcClient;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_get_quote_integration() {
        let sol_mint = jupiter::get_token_mint("SOL").unwrap();
        let usdc_mint = jupiter::get_token_mint("USDC").unwrap();
        let amount = 1_000_000; // 1 USDC

        let quote = jupiter::get_quote(&usdc_mint, &sol_mint, amount).await;

        assert!(quote.is_ok(), "Failed to get quote: {:?}", quote.err());

        let quote = quote.unwrap();
        assert!(quote.out_amount > 0, "Output amount should be positive");
        println!("Quote: {} USDC → {} SOL lamports", amount, quote.out_amount);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_fee_optimizer_integration() {
        // Use Solana devnet RPC for networked test
        let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());

        let fee = fee_optimizer::get_optimal_priority_fee(&rpc, fee_optimizer::FeeStrategy::Standard)
            .await;

        assert!(fee.is_ok(), "Failed to get optimal fee: {:?}", fee.err());

        let fee = fee.unwrap();
        assert!(fee > 0, "Fee should be positive");
        println!("Optimal priority fee: {} micro-lamports/CU", fee);
    }

    #[test]
    fn test_token_mints() {
        let sol = jupiter::get_token_mint("SOL").unwrap();
        assert_eq!(
            sol.to_string(),
            "So11111111111111111111111111111111111111112"
        );

        let usdc = jupiter::get_token_mint("USDC").unwrap();
        assert_eq!(
            usdc.to_string(),
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
        );
    }
}
