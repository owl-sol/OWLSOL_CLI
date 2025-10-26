use owlsol_cli::core::config::load_wallet;
use owlsol_cli::core::jupiter::{get_token_mint, ultra_swap_order};
use owlsol_cli::core::transaction::send_optimized_transaction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_real_sol_to_usdc_swap() {
    // Load user's devnet keypair
    let keypair = load_wallet().expect("Failed to load keypair");
    let user_pubkey = keypair.pubkey();

    // Prepare swap: SOL → USDC
    let sol_mint = get_token_mint("SOL").unwrap();
    let usdc_mint = get_token_mint("USDC").unwrap();
    let amount = 1_000_000; // 0.001 SOL
    let order = ultra_swap_order(&sol_mint, &usdc_mint, amount, &user_pubkey)
        .await
        .expect("Ultra swap order failed");
    assert!(order.error.is_none(), "Ultra API error: {:?}", order.error);
    assert!(!order.swap_transaction.is_empty(), "No swap transaction returned");

    // Send transaction to devnet
    let rpc_url = "https://api.testnet.solana.com";
    let rpc = solana_client::rpc_client::RpcClient::new(rpc_url.to_string());
    let priority_fee = 0;
    let result = send_optimized_transaction(&rpc, &keypair, &order.swap_transaction, priority_fee).await;
    match result {
        Ok(sig) => println!("Swap sent! Signature: {}", sig),
        Err(e) => panic!("Swap failed: {}", e),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_real_usdc_to_sol_swap() {
    // Load user's devnet keypair
    let keypair = load_wallet().expect("Failed to load keypair");
    let user_pubkey = keypair.pubkey();

    // Prepare swap: USDC → SOL
    let sol_mint = get_token_mint("SOL").unwrap();
    let usdc_mint = get_token_mint("USDC").unwrap();
    let amount = 1_000_000; // 1 USDC
    let order = ultra_swap_order(&usdc_mint, &sol_mint, amount, &user_pubkey)
        .await
        .expect("Ultra swap order failed");
    assert!(order.error.is_none(), "Ultra API error: {:?}", order.error);
    assert!(!order.swap_transaction.is_empty(), "No swap transaction returned");

    // Send transaction to devnet
    let rpc_url = "https://api.testnet.solana.com";
    let rpc = solana_client::rpc_client::RpcClient::new(rpc_url.to_string());
    let priority_fee = 0;
    let result = send_optimized_transaction(&rpc, &keypair, &order.swap_transaction, priority_fee).await;
    match result {
        Ok(sig) => println!("Swap sent! Signature: {}", sig),
        Err(e) => panic!("Swap failed: {}", e),
    }
}
