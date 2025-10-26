use thiserror::Error;

pub type Result<T> = std::result::Result<T, SolanaError>;

#[derive(Error, Debug)]
pub enum SolanaError {
    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Account not found: {0}")]
    AccountNotFound(String),

    #[error("Insufficient funds: required {0} SOL")]
    InsufficientFunds(f64),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Invalid keypair")]
    InvalidKeypair,

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Solana SDK error: {0}")]
    SdkError(String),

    #[error("Program error: {0}")]
    ProgramError(String),

    #[error("Invalid account data")]
    InvalidAccountData,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
