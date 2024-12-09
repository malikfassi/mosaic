use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid color format")]
    InvalidColorFormat {},

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Color change cooldown in effect")]
    ColorChangeCooldown {},

    #[error("Not the pixel owner")]
    NotPixelOwner {},

    #[error("Pixel not found")]
    PixelNotFound {},
} 