use solana_sdk::{
    pubkey::Pubkey,
    system_instruction,
};
/// Send a simple SOL transfer transaction (for fee optimization testing)
pub async fn send_sol_transfer(
    rpc: &RpcClient,
    payer: &Keypair,
    recipient: &Pubkey,
    lamports: u64,
    with_compression: bool,
) -> Result<String> {
    let recent_blockhash = rpc.get_latest_blockhash().context("Failed to get recent blockhash")?;
    let mut instructions = vec![system_instruction::transfer(&payer.pubkey(), recipient, lamports)];
    // Optionally add a dummy instruction to simulate unoptimized data
    if !with_compression {
        // Add a no-op instruction to increase transaction size (simulate unoptimized)
        instructions.push(system_instruction::transfer(&payer.pubkey(), recipient, 0));
    }
    let tx = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    let signature = rpc.send_and_confirm_transaction(&tx).context("Failed to send SOL transfer")?;
    Ok(signature.to_string())
}
use anyhow::{Context, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::{Transaction, VersionedTransaction},
};
use base64::Engine as _;

/// Sign and send a Jupiter-provided base64 swap transaction
/// Note: Jupiter includes compute budget instructions in its prepared message.
/// We sign the message with the payer and submit it. Priority fee value is kept
/// for future customization but not injected here (Jupiter recommends building
/// compute budget into the quote).
pub async fn send_optimized_transaction(
    rpc: &RpcClient,
    payer: &Keypair,
    base64_tx: &str,
    _priority_fee: u64,
) -> Result<String> {
    let tx_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_tx)
        .context("Failed to decode base64 transaction")?;

    // Try to interpret as VersionedTransaction first (Jupiter v6 default)
    if let Ok(mut vtx) = bincode::deserialize::<VersionedTransaction>(&tx_bytes) {
        // Sign the versioned message with the payer key
        let msg_bytes = vtx.message.serialize();
        let sig = payer.sign_message(&msg_bytes);
        // Replace the first required signature. If more signers are required, Jupiter
        // would expect them to be present; here we assume single-signer flow.
        if vtx.signatures.is_empty() {
            vtx.signatures = vec![sig];
        } else {
            vtx.signatures[0] = sig;
        }

        let signature = rpc
            .send_and_confirm_transaction(&vtx)
            .context("Failed to send versioned transaction")?;
        return Ok(signature.to_string());
    }

    // Fallback: legacy Transaction
    if let Ok(mut ltx) = bincode::deserialize::<Transaction>(&tx_bytes) {
        // Try to (re)sign with a fresh blockhash to avoid expiry
        let recent_blockhash = rpc
            .get_latest_blockhash()
            .context("Failed to get recent blockhash")?;
        ltx.sign(&[payer], recent_blockhash);

        let signature = rpc
            .send_and_confirm_transaction(&ltx)
            .context("Failed to send legacy transaction")?;
        return Ok(signature.to_string());
    }

    // If neither deserialization works, return an error
    anyhow::bail!("Unsupported transaction format: cannot deserialize Jupiter transaction")
}

/// Decode a base64-encoded transaction and attempt to deserialize it as a
/// VersionedTransaction (preferred) or legacy Transaction. Returns Ok(()) if
/// decoding and deserialization succeed, or an error otherwise. The
/// `priority_fee` parameter is reserved for future optimization hooks.
pub fn decode_and_optimize_transaction(base64_tx: &str, _priority_fee: u64) -> Result<()> {
    let tx_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_tx)
        .context("Failed to decode base64 transaction")?;

    // Try versioned first
    if bincode::deserialize::<VersionedTransaction>(&tx_bytes).is_ok() {
        return Ok(());
    }

    // Fallback to legacy
    if bincode::deserialize::<Transaction>(&tx_bytes).is_ok() {
        return Ok(());
    }

    anyhow::bail!("Unsupported transaction format: cannot deserialize transaction")
}

/// Simulate transaction to estimate compute units (legacy only)
pub async fn simulate_transaction(rpc: &RpcClient, tx: &Transaction) -> Result<u64> {
    let simulation = rpc
        .simulate_transaction(tx)
        .context("Failed to simulate transaction")?;

    if let Some(err) = simulation.value.err {
        anyhow::bail!("Transaction simulation failed: {:?}", err);
    }

    let units_consumed = simulation
        .value
        .units_consumed
        .context("No compute units data in simulation")?;

    Ok(units_consumed)
}

#[cfg(test)]
mod tests {
    use base64::Engine as _;
    #[test]
    fn test_decode_base64_failure() {
        // Empty base64 should fail decode path and thus send_raw_transaction
        // would also fail at runtime; here we just ensure function exists.
        let bytes = base64::engine::general_purpose::STANDARD.decode("!!!notbase64!!!");
        assert!(bytes.is_err(), "Decoding invalid base64 should error");
    }
}
