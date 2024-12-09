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

    #[error("Invalid coordinates: x={x}, y={y}")]
    InvalidCoordinates { x: u32, y: u32 },

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Color change cooldown not elapsed")]
    ColorChangeCooldown {},

    #[error("Pixel not found")]
    PixelNotFound {},

    #[error("Not pixel owner")]
    NotPixelOwner {},
} 