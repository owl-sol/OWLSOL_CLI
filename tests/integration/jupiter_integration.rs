#[cfg(test)]
mod jupiter_integration_tests {
    use owlsol_cli::core::jupiter::*;
    use solana_sdk::pubkey::Pubkey;
    use tokio;
    use std::net::ToSocketAddrs;

    fn jupiter_host_reachable() -> bool {
        "quote-api.jup.ag:443"
            .to_socket_addrs()
            .map(|mut addrs| addrs.next().is_some())
            .unwrap_or(false)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_jupiter_quote_devnet_skip_if_unreachable() {
        if std::env::var("RUN_JUPITER_TESTS").unwrap_or_default() != "1" && !jupiter_host_reachable() {
            println!("Jupiter host unreachable, skipping test.");
            return;
        }
        let sol_mint = get_token_mint("SOL").unwrap();
        let usdc_mint = get_token_mint("USDC").unwrap();
        let amount = 1_000_000;
        let quote = get_quote(&usdc_mint, &sol_mint, amount).await;
        assert!(quote.is_ok(), "Quote request should succeed");
        let quote = quote.unwrap();
        assert!(quote.out_amount > 0, "Output amount should be positive");
        assert_eq!(quote.input_mint, usdc_mint.to_string());
        assert_eq!(quote.output_mint, sol_mint.to_string());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_real_sol_swap_on_devnet() {
        use owlsol_cli::core::config::load_keypair;
        use owlsol_cli::core::transaction::send_optimized_transaction;
        use solana_sdk::pubkey::Pubkey;
        use std::env;

        // Load user's devnet keypair (update path as needed)
        let keypair_path = env::var("DEVNET_KEYPAIR").unwrap_or_else(|_| "~/.config/solana/id.json".to_string());
        let keypair = load_keypair(&keypair_path).expect("Failed to load keypair");
        let user_pubkey = keypair.pubkey();

        // Prepare swap: SOL → USDC
        let sol_mint = get_token_mint("SOL").unwrap();
        let usdc_mint = get_token_mint("USDC").unwrap();
        let amount = 1_000_000; // 0.001 SOL
        let quote = get_quote(&sol_mint, &usdc_mint, amount).await.expect("Quote failed");
        let swap = get_swap_transaction(&quote, &user_pubkey).await.expect("Swap tx failed");

        // Send transaction to devnet
        let result = send_optimized_transaction(&swap.swap_transaction, &keypair, None).await;
        match result {
            Ok(sig) => println!("Swap sent! Signature: {}", sig),
            Err(e) => panic!("Swap failed: {}", e),
        }
    }
}
