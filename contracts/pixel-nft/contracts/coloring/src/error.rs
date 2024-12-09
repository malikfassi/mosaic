use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Color change is in cooldown period")]
    ColorChangeCooldown {},

    #[error("Invalid color format")]
    InvalidColorFormat {},

    #[error("Pixel not found")]
    PixelNotFound {},

    #[error("Not the pixel owner")]
    NotPixelOwner {},
} 