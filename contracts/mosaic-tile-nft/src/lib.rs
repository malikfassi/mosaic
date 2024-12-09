pub mod contract;
pub mod error;
pub mod msg;
pub mod state;

#[cfg(test)]
mod testing;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, QueryMsg};
pub use crate::state::{Color, Position, TileMetadata};
pub use cw721_base::msg::InstantiateMsg;

// Re-export entry points for other packages to use
pub use crate::contract::{execute, instantiate, query};
