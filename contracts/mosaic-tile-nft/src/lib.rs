pub mod contract;
pub mod error;
pub mod msg;
pub mod state;
pub mod execute;
pub mod query;
pub mod helpers;

#[cfg(test)]
mod testing;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
pub use crate::state::{Color, Position, TileMetadata};

// Re-export entry points for other packages to use
pub use crate::contract::{execute, instantiate, query};
