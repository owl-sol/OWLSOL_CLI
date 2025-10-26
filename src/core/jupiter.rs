use anyhow::{Context, Result};
use serde::{de, Deserialize, Deserializer, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use serde_json::json;

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

/// Ultra API order response
#[derive(Debug, Deserialize)]
pub struct UltraOrderResponse {
    #[serde(rename = "swapTransaction")]
    pub swap_transaction: String,
    #[serde(rename = "error")]
    pub error: Option<String>,
    #[serde(rename = "message")]
    pub message: Option<String>,
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

/// Get swap transaction from Jupiter Ultra API
pub async fn ultra_swap_order(
    input_mint: &Pubkey,
    output_mint: &Pubkey,
    amount: u64,
    user_public_key: &Pubkey,
) -> Result<UltraOrderResponse> {
    let url = "https://lite-api.jup.ag/ultra/v1/order";
    let client = reqwest::Client::new();
    let req_body = json!({
        "inputMint": input_mint.to_string(),
        "outputMint": output_mint.to_string(),
        "amount": amount.to_string(),
        "userPublicKey": user_public_key.to_string(),
        "swapMode": "ExactIn",
        "slippageBps": 50,
        "feeBps": 0,
        "asLegacyTransaction": false,
        "wrapAndUnwrapSol": true,
        "dynamicComputeUnitLimit": true,
        "useSharedAccounts": false,
        "onlyDirectRoutes": false,
        "enableDexes": [],
        "disableDexes": [],
        "maxAccounts": 32
    });

    let response = client
        .post(url)
        .json(&req_body)
        .send()
        .await
        .context("Failed to fetch Ultra swap order from Jupiter")?;

    let status = response.status();
    let resp_text = response.text().await.context("Failed to read Ultra API response")?;
    if !status.is_success() {
        anyhow::bail!("Ultra API error: {}", resp_text);
    }

    let order: UltraOrderResponse = serde_json::from_str(&resp_text)
        .context("Failed to parse Ultra API order response")?;
    if order.swap_transaction.is_empty() {
        let msg = order.message.unwrap_or_else(|| "Unknown error".to_string());
        anyhow::bail!("Ultra API error: {}", msg);
    }
    Ok(order)
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
