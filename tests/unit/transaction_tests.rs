#[cfg(test)]
mod transaction_unit_tests {
    use owlsol_cli::core::transaction::*;
    use base64;

    #[test]
    fn test_decode_and_optimize_transaction_valid_base64() {
        // This is a deterministic base64 string for a trivial transaction
        let base64_tx = "AAECAwQFBgcICQoLDA0ODw==";
        let priority_fee = 5000;
        let result = decode_and_optimize_transaction(base64_tx, priority_fee);
        assert!(result.is_ok());
    }

    #[test]
    fn test_decode_and_optimize_transaction_invalid_base64() {
        let base64_tx = "not_base64!";
        let priority_fee = 5000;
        let result = decode_and_optimize_transaction(base64_tx, priority_fee);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Base64 decode error"));
    }

    #[test]
    fn test_decode_and_optimize_transaction_empty_string() {
        let base64_tx = "";
        let priority_fee = 5000;
        let result = decode_and_optimize_transaction(base64_tx, priority_fee);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_and_optimize_transaction_zero_fee() {
        let base64_tx = "AAECAwQFBgcICQoLDA0ODw==";
        let priority_fee = 0;
        let result = decode_and_optimize_transaction(base64_tx, priority_fee);
        assert!(result.is_ok());
    }
}
