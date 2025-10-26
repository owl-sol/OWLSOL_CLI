use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Get Jupiter's public Address Lookup Table
/// This ALT contains common token accounts and reduces TX size by ~46%
pub fn get_jupiter_alt() -> Pubkey {
    Pubkey::from_str("D1ZN9Wj1fRSUQfCjhvnu152PFDaVmk5kHBfavRPVtRgC")
        .expect("Invalid Jupiter ALT address")
}

/// Check if using ALT will benefit this transaction
pub fn should_use_alt(num_accounts: usize) -> bool {
    // ALT is beneficial when we have more than 10 accounts
    // Below that, the overhead might not be worth it
    num_accounts > 10
}

/// Calculate expected byte savings from using ALT
pub fn calculate_alt_savings(num_accounts: usize) -> usize {
    if num_accounts <= 10 {
        return 0;
    }

    // Each account in ALT:
    // - Without ALT: 32 bytes (full pubkey)
    // - With ALT: 1 byte (index)
    // Savings: 31 bytes per account

    let accounts_in_alt = num_accounts - 5; // Assume 5 accounts not in ALT
    accounts_in_alt * 31
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_jupiter_alt() {
        let alt = get_jupiter_alt();
        assert_eq!(
            alt.to_string(),
            "D1ZN9Wj1fRSUQfCjhvnu152PFDaVmk5kHBfavRPVtRgC"
        );
    }

    #[test]
    fn test_calculate_alt_savings() {
        // Typical Jupiter swap: 30 accounts
        let savings = calculate_alt_savings(30);
        // (30 - 5) × 31 = 775 bytes
        assert_eq!(savings, 775);
    }

    #[test]
    fn test_should_use_alt() {
        assert!(!should_use_alt(5));
        assert!(!should_use_alt(10));
        assert!(should_use_alt(15));
        assert!(should_use_alt(30));
    }
}
