use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Cw721(#[from] cw721_base::ContractError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid tile ID: {tile_id}")]
    InvalidTileId { tile_id: u32 },

    #[error("Invalid pixel ID: {pixel_id}")]
    InvalidPixelId { pixel_id: u32 },

    #[error("Invalid position: ({x}, {y})")]
    InvalidPosition { x: u32, y: u32 },

    #[error("Position ({x}, {y}) is already taken")]
    PositionTaken { x: u32, y: u32 },

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Batch size exceeds maximum of {max}")]
    BatchTooLarge { max: u32 },
}
