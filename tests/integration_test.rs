use owlsol_cli::core::config::load_wallet;
use owlsol_cli::core::transaction::send_sol_transfer;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_sol_transfer_fee_optimization() {
    let keypair = load_wallet().expect("Failed to load keypair");
    let rpc_url = "https://api.devnet.solana.com";
    let rpc = RpcClient::new(rpc_url.to_string());
    let recipient = Pubkey::new_unique();
    let lamports = 100_000;

    // Transfer with compression (optimized)
    let sig_compressed = send_sol_transfer(&rpc, &keypair, &recipient, lamports, true)
        .await
        .expect("Compressed transfer failed");
    println!("Compressed transfer signature: {}", sig_compressed);

    // Transfer without compression (unoptimized)
    let sig_uncompressed = send_sol_transfer(&rpc, &keypair, &recipient, lamports, false)
        .await
        .expect("Uncompressed transfer failed");
    println!("Uncompressed transfer signature: {}", sig_uncompressed);

    // Fetch fee info for both transactions
    let fee_compressed = rpc.get_fee_for_message(&rpc.get_transaction(&sig_compressed, solana_client::rpc_config::RpcTransactionConfig { encoding: Some(solana_transaction_status::UiTransactionEncoding::Base64), commitment: None, max_supported_transaction_version: None }).unwrap().transaction.message).unwrap();
    let fee_uncompressed = rpc.get_fee_for_message(&rpc.get_transaction(&sig_uncompressed, solana_client::rpc_config::RpcTransactionConfig { encoding: Some(solana_transaction_status::UiTransactionEncoding::Base64), commitment: None, max_supported_transaction_version: None }).unwrap().transaction.message).unwrap();
    println!("Fee (compressed): {} lamports", fee_compressed);
    println!("Fee (uncompressed): {} lamports", fee_uncompressed);
    assert!(fee_compressed <= fee_uncompressed, "Compression should not increase fee");
}
#[cfg(test)]
mod tests {
    use owlsol_cli::core::{fee_optimizer, jupiter};
    use solana_client::rpc_client::RpcClient;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_get_quote_integration() {
        // Skip gracefully if we don't have internet/DNS to reach Jupiter
        if std::env::var("RUN_JUPITER_TESTS").unwrap_or_default() != "1" {
            // Quick DNS/port probe with a short timeout
            use std::net::{TcpStream, ToSocketAddrs};
            use std::time::Duration;
            let addr = ("quote-api.jup.ag", 443)
                .to_socket_addrs()
                .ok()
                .and_then(|mut it| it.next());
            let reachable = addr
                .and_then(|a| TcpStream::connect_timeout(&a, Duration::from_millis(800)).ok())
                .is_some();
            if !reachable {
                eprintln!("skipping test_get_quote_integration: Jupiter host not reachable (set RUN_JUPITER_TESTS=1 to force) ");
                return;
            }
        }

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
