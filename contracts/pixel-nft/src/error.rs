use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unknown reply id: {id}")]
    UnknownReplyId { id: u64 },

    #[error("Contract not initialized")]
    ContractNotInitialized {},

    #[error("Invalid contract address")]
    InvalidContractAddress {},

    #[error("Invalid coordinates: x={x}, y={y}")]
    InvalidCoordinates { x: u32, y: u32 },

    #[error("Insufficient funds")]
    InsufficientFunds {},
} 