use anyhow::{Context, Result};
use serde::{de, Deserialize, Deserializer, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Helper to deserialize u64 that may be encoded as a JSON string or number
fn de_u64_string_or_number<'de, D>(deserializer: D) -> std::result::Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum U64StrOrNum {
        Str(String),
        Num(u64),
    }

    match U64StrOrNum::deserialize(deserializer)? {
        U64StrOrNum::Str(s) => s
            .parse::<u64>()
            .map_err(|e| de::Error::custom(format!("failed to parse u64: {e}"))),
        U64StrOrNum::Num(n) => Ok(n),
    }
}

/// Jupiter quote response (subset we need)
#[derive(Debug, Deserialize)]
pub struct QuoteResponse {
    #[serde(rename = "inputMint")]
    pub input_mint: String,

    #[serde(rename = "inAmount")]
    pub in_amount: String,

    #[serde(rename = "outputMint")]
    pub output_mint: String,

    #[serde(rename = "outAmount", deserialize_with = "de_u64_string_or_number")]
    pub out_amount: u64,

    #[serde(
        rename = "otherAmountThreshold",
        deserialize_with = "de_u64_string_or_number"
    )]
    pub other_amount_threshold: u64,

    #[serde(rename = "swapMode")]
    pub swap_mode: String,

    #[serde(rename = "slippageBps")]
    pub slippage_bps: u16,

    #[serde(rename = "priceImpactPct")]
    pub price_impact_pct: String,

    #[serde(default)]
    pub routes_count: Option<usize>,
}

/// Jupiter swap transaction response
#[derive(Debug, Deserialize)]
pub struct SwapResponse {
    #[serde(rename = "swapTransaction")]
    pub swap_transaction: String,

    #[serde(rename = "lastValidBlockHeight")]
    pub last_valid_block_height: Option<u64>,
}

/// Get token mint address by symbol
pub fn get_token_mint(symbol: &str) -> Result<Pubkey> {
    let mint_str = match symbol.to_uppercase().as_str() {
        "SOL" => "So11111111111111111111111111111111111111112",
        "USDC" => "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "USDT" => "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
        "BONK" => "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
        "WIF" => "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm",
        _ => anyhow::bail!("Unsupported token: {}", symbol),
    };

    Pubkey::from_str(mint_str).context("Invalid mint address")
}

/// Get quote from Jupiter API
pub async fn get_quote(input_mint: &Pubkey, output_mint: &Pubkey, amount: u64) -> Result<QuoteResponse> {
    let url = format!(
        "https://quote-api.jup.ag/v6/quote?inputMint={}&outputMint={}&amount={}&slippageBps=50",
        input_mint, output_mint, amount
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch quote from Jupiter")?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("Jupiter API error: {}", error_text);
    }

    let quote: QuoteResponse = response
        .json()
        .await
        .context("Failed to parse Jupiter quote response")?;

    Ok(quote)
}

/// Get swap transaction from Jupiter API
pub async fn get_swap_transaction(quote: &QuoteResponse, user_public_key: &Pubkey) -> Result<SwapResponse> {
    let url = "https://quote-api.jup.ag/v6/swap";

    #[derive(Serialize)]
    struct SwapRequest {
        #[serde(rename = "quoteResponse")]
        quote_response: serde_json::Value,
        #[serde(rename = "userPublicKey")]
        user_public_key: String,
        #[serde(rename = "wrapAndUnwrapSol")]
        wrap_and_unwrap_sol: bool,
    }

    let quote_json = serde_json::to_value(quote).context("Failed to serialize quote")?;

    let request = SwapRequest {
        quote_response: quote_json,
        user_public_key: user_public_key.to_string(),
        wrap_and_unwrap_sol: true,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .json(&request)
        .send()
        .await
        .context("Failed to get swap transaction from Jupiter")?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("Jupiter swap API error: {}", error_text);
    }

    let swap_response: SwapResponse = response
        .json()
        .await
        .context("Failed to parse Jupiter swap response")?;

    Ok(swap_response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token_mint() {
        let sol_mint = get_token_mint("SOL").unwrap();
        assert_eq!(
            sol_mint.to_string(),
            "So11111111111111111111111111111111111111112"
        );

        let usdc_mint = get_token_mint("USDC").unwrap();
        assert_eq!(
            usdc_mint.to_string(),
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
        );
    }
}
