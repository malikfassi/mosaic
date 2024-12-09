use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid position: x={x}, y={y}")]
    InvalidPosition { x: u32, y: u32 },

    #[error("Position already minted: x={x}, y={y}")]
    PositionTaken { x: u32, y: u32 },

    #[error("Random minting is disabled")]
    RandomMintingDisabled {},

    #[error("Position minting is disabled")]
    PositionMintingDisabled {},

    #[error("Batch size exceeds maximum")]
    BatchSizeExceeded {},

    #[error("Colors count does not match mint count")]
    ColorCountMismatch {},

    #[error("No available positions for random minting")]
    NoAvailablePositions {},

    #[error("Invalid payment amount")]
    InvalidPayment {},

    #[error("Invalid configuration update")]
    InvalidConfigUpdate {},
} 