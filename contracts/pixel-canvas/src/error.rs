use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid pixel coordinates: x={x}, y={y}")]
    InvalidPixelCoordinates { x: u32, y: u32 },

    #[error("Pixel already owned")]
    PixelAlreadyOwned {},

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Invalid color format")]
    InvalidColorFormat {},

    #[error("You don't own this pixel")]
    NotPixelOwner {},
} 