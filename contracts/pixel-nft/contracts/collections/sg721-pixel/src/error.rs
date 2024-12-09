use cosmwasm_std::StdError;
use cw721_base::ContractError as Cw721ContractError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Base(#[from] Cw721ContractError),

    #[error("Invalid pixel coordinates: x={x}, y={y}")]
    InvalidPixelCoordinates { x: u32, y: u32 },

    #[error("Pixel at coordinates (x={x}, y={y}) already exists")]
    PixelAlreadyExists { x: u32, y: u32 },

    #[error("Invalid color format: {color}")]
    InvalidColorFormat { color: String },

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Token ID does not match pixel coordinates")]
    TokenIdMismatch {},
} 