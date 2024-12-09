use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid tile position: x={x}, y={y}")]
    InvalidPosition { x: u32, y: u32 },

    #[error("Rate limit exceeded. Try again in {seconds} seconds")]
    RateLimitExceeded { seconds: u64 },

    #[error("Insufficient payment. Required: {required}, sent: {sent}")]
    InsufficientPayment { required: u128, sent: u128 },

    #[error("Permission expired")]
    PermissionExpired {},

    #[error("Public editing disabled for this tile")]
    PublicEditingDisabled {},

    #[error("Invalid color value")]
    InvalidColor {},

    #[error("Tile not found in NFT contract")]
    TileNotFound {},

    #[error("Color change not allowed")]
    ColorChangeNotAllowed {},

    #[error("Invalid configuration update")]
    InvalidConfigUpdate {},

    #[error("Permission already granted to {address}")]
    PermissionAlreadyGranted { address: String },

    #[error("Permission not found for {address}")]
    PermissionNotFound { address: String },

    #[error("Invalid expiry time")]
    InvalidExpiryTime {},

    #[error("Invalid fee amount")]
    InvalidFeeAmount {},

    #[error("Contract paused")]
    ContractPaused {},
} 