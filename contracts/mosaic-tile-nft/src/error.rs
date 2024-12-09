use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use sg1::FeeError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    Base(#[from] sg721_base::ContractError),

    #[error("{0}")]
    Fee(#[from] FeeError),

    #[error("Token not found")]
    TokenNotFound {},

    #[error("Token metadata is frozen")]
    TokenMetadataFrozen {},

    #[error("Token metadata is already frozen")]
    TokenMetadataAlreadyFrozen {},

    #[error("Not enabled for updates")]
    NotEnableUpdatable {},

    #[error("Already enabled for updates")]
    AlreadyEnableUpdatable {},

    #[error("Unauthorized")]
    Unauthorized {},

    // Tile-specific errors
    #[error("Position {x},{y} is already taken")]
    PositionTaken { x: u32, y: u32 },

    #[error("Invalid position: x and y must be within bounds")]
    InvalidPosition {},

    #[error("Invalid color values")]
    InvalidColor {},

    #[error("Color update not allowed")]
    ColorUpdateNotAllowed {},

    #[error("Invalid token ID format")]
    InvalidTokenId {},

    #[error("Token ID already exists")]
    TokenIdAlreadyExists {},

    #[error("Position mismatch: token already exists at different position")]
    PositionMismatch {},

    #[error("History limit exceeded")]
    HistoryLimitExceeded {},

    #[error("Invalid state transition")]
    InvalidStateTransition {},

    #[error("Operation not allowed in current state")]
    InvalidOperationForState {},
}
