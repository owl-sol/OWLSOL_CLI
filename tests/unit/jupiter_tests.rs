#[cfg(test)]
mod jupiter_unit_tests {
    use owlsol_cli::core::jupiter::*;
    use solana_sdk::pubkey::Pubkey;

    // TOKEN MINT TESTS
    #[test]
    fn test_get_token_mint_sol() {
        let mint = get_token_mint("SOL").unwrap();
        assert_eq!(
            mint.to_string(),
            "So11111111111111111111111111111111111111112"
        );
    }

    #[test]
    fn test_get_token_mint_usdc() {
        let mint = get_token_mint("USDC").unwrap();
        assert_eq!(
            mint.to_string(),
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
        );
    }

    #[test]
    fn test_get_token_mint_usdt() {
        let mint = get_token_mint("USDT").unwrap();
        assert_eq!(
            mint.to_string(),
            "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"
        );
    }

    #[test]
    fn test_get_token_mint_bonk() {
        let mint = get_token_mint("BONK").unwrap();
        assert_eq!(
            mint.to_string(),
            "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"
        );
    }

    #[test]
    fn test_get_token_mint_wif() {
        let mint = get_token_mint("WIF").unwrap();
        assert_eq!(
            mint.to_string(),
            "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"
        );
    }

    #[test]
    fn test_get_token_mint_case_insensitive() {
        let mint_lower = get_token_mint("sol").unwrap();
        let mint_upper = get_token_mint("SOL").unwrap();
        let mint_mixed = get_token_mint("SoL").unwrap();
        assert_eq!(mint_lower, mint_upper);
        assert_eq!(mint_lower, mint_mixed);
    }

    #[test]
    fn test_get_token_mint_invalid_token() {
        let result = get_token_mint("INVALID_TOKEN");
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Unsupported token"));
    }

    #[test]
    fn test_get_token_mint_empty_string() {
        let result = get_token_mint("");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_token_mint_special_characters() {
        assert!(get_token_mint("$$$").is_err());
        assert!(get_token_mint("SOL!").is_err());
        assert!(get_token_mint("@SOL").is_err());
        assert!(get_token_mint("SOL#USD").is_err());
    }

    #[test]
    fn test_get_token_mint_whitespace() {
        assert!(get_token_mint(" SOL ").is_err());
        assert!(get_token_mint("SOL ").is_err());
        assert!(get_token_mint(" SOL").is_err());
    }

    #[test]
    fn test_get_token_mint_numbers() {
        assert!(get_token_mint("SOL123").is_err());
        assert!(get_token_mint("123").is_err());
    }

    #[test]
    fn test_all_supported_tokens() {
        let tokens = vec!["SOL", "USDC", "USDT", "BONK", "WIF"];
        for token in tokens {
            let result = get_token_mint(token);
            assert!(result.is_ok(), "Token {} should be supported", token);
            let pubkey = result.unwrap();
            assert_eq!(pubkey.to_bytes().len(), 32);
        }
    }

    // QUOTE API TESTS (Network Required)
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_get_quote_valid_params() {
        let sol_mint = get_token_mint("SOL").unwrap();
        let usdc_mint = get_token_mint("USDC").unwrap();
        let amount = 1_000_000; // 1 USDC
        let quote = get_quote(&usdc_mint, &sol_mint, amount).await;
        assert!(quote.is_ok(), "Quote request should succeed");
        let quote = quote.unwrap();
        assert!(quote.out_amount > 0, "Output amount should be positive");
        assert_eq!(quote.input_mint, usdc_mint.to_string());
        assert_eq!(quote.output_mint, sol_mint.to_string());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_get_quote_zero_amount() {
        let sol_mint = get_token_mint("SOL").unwrap();
        let usdc_mint = get_token_mint("USDC").unwrap();
        let amount = 0;
        let quote = get_quote(&usdc_mint, &sol_mint, amount).await;
        assert!(quote.is_err(), "Zero amount should fail");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_get_quote_very_small_amount() {
        let sol_mint = get_token_mint("SOL").unwrap();
        let usdc_mint = get_token_mint("USDC").unwrap();
        let amount = 1; // 0.000001 USDC
        let quote = get_quote(&usdc_mint, &sol_mint, amount).await;
        if quote.is_err() {
            let error = quote.unwrap_err().to_string();
            assert!(!error.is_empty(), "Should have error message");
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_get_quote_very_large_amount() {
        let sol_mint = get_token_mint("SOL").unwrap();
        let usdc_mint = get_token_mint("USDC").unwrap();
        let amount = 1_000_000_000_000; // 1 million USDC
        let quote = get_quote(&usdc_mint, &sol_mint, amount).await;
        if quote.is_err() {
            let error = quote.unwrap_err().to_string();
            assert!(
                error.contains("liquidity") || error.contains("API error") || !error.is_empty(),
                "Should fail gracefully"
            );
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_get_quote_same_token() {
        let sol_mint = get_token_mint("SOL").unwrap();
        let amount = 1_000_000;
        let quote = get_quote(&sol_mint, &sol_mint, amount).await;
        assert!(quote.is_err());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_get_quote_all_pairs() {
        let tokens = vec![
            ("SOL", "USDC"),
            ("USDC", "SOL"),
            ("SOL", "USDT"),
            ("USDC", "USDT"),
        ];
        for (from, to) in tokens {
            let from_mint = get_token_mint(from).unwrap();
            let to_mint = get_token_mint(to).unwrap();
            let amount = 1_000_000;
            let quote = get_quote(&from_mint, &to_mint, amount).await;
            assert!(quote.is_ok(), "Quote for {} → {} should succeed", from, to);
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_get_quote_response_structure() {
        let sol_mint = get_token_mint("SOL").unwrap();
        let usdc_mint = get_token_mint("USDC").unwrap();
        let amount = 1_000_000;
        let quote = get_quote(&usdc_mint, &sol_mint, amount).await.unwrap();
        assert!(!quote.input_mint.is_empty());
        assert!(!quote.output_mint.is_empty());
        assert!(!quote.in_amount.is_empty());
        assert!(quote.out_amount > 0);
        assert!(quote.slippage_bps > 0);
        let price_impact: f64 = quote.price_impact_pct.parse().unwrap_or(100.0);
        assert!(price_impact < 50.0, "Price impact too high: {}%", price_impact);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    async fn test_get_swap_transaction_valid_quote() {
        let sol_mint = get_token_mint("SOL").unwrap();
        let usdc_mint = get_token_mint("USDC").unwrap();
        let amount = 1_000_000;
        let quote = get_quote(&usdc_mint, &sol_mint, amount).await.unwrap();
        let user_pubkey = Pubkey::new_unique();
        let swap_response = get_swap_transaction(&quote, &user_pubkey).await;
        assert!(swap_response.is_ok(), "Swap transaction should be generated");
        let swap = swap_response.unwrap();
        assert!(!swap.swap_transaction.is_empty(), "Transaction should not be empty");
    }
}
