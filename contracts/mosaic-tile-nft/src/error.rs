use cosmwasm_std::StdError;
use cw721_base::ContractError as Cw721ContractError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Position {x}, {y} is already taken")]
    PositionTaken { x: u32, y: u32 },

    #[error("Position {x}, {y} is out of bounds")]
    PositionOutOfBounds { x: u32, y: u32 },

    #[error("Token metadata is frozen")]
    TokenMetadataFrozen {},

    #[error("Token metadata is already frozen")]
    TokenMetadataAlreadyFrozen {},

    #[error("Already enable updatable")]
    AlreadyEnableUpdatable {},

    #[error("Base contract error: {0}")]
    Base(#[from] Cw721ContractError),
}
