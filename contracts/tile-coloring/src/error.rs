use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Permission already granted to {address}")]
    PermissionAlreadyGranted { address: String },

    #[error("Permission not found for {address}")]
    PermissionNotFound { address: String },

    #[error("Rate limit exceeded. Try again in {seconds} seconds")]
    RateLimitExceeded { seconds: u64 },

    #[error("Insufficient payment. Required: {required}, sent: {sent}")]
    InsufficientPayment { required: u128, sent: u128 },

    #[error("Invalid token ID format")]
    InvalidTokenId {},

    #[error("NFT ownership verification failed")]
    OwnershipVerificationFailed {},

    #[error("Invalid position: {x}, {y}")]
    InvalidPosition { x: u32, y: u32 },

    #[error("Invalid color values")]
    InvalidColor {},

    #[error("Invalid fee amount")]
    InvalidFeeAmount {},

    #[error("Invalid time window")]
    InvalidTimeWindow {},

    #[error("Invalid batch size")]
    InvalidBatchSize {},

    #[error("Permission expired")]
    PermissionExpired {},

    #[error("Feature disabled")]
    FeatureDisabled {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
} 