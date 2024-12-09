pub mod contract;
mod error;
pub mod msg;
mod state;
pub mod upgrades;

pub use crate::error::ContractError;
pub use crate::state::Sg721PixelContract;

// Define our pixel extension
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PixelExtension {
    pub x: u32,
    pub y: u32,
    pub color: String,
}

// Export message types
pub type ExecuteMsg = sg721::ExecuteMsg<PixelExtension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

pub mod entry {
    use super::*;
    use crate::state::Sg721PixelContract;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Empty};
    use cw2::set_contract_version;
    use sg721::InstantiateMsg;

    // version info for migration info
    pub const CONTRACT_NAME: &str = "crates.io:sg721-pixel";
    pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

    #[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let res = Sg721PixelContract::default().instantiate(deps, env, info, msg)?;

        Ok(res
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Sg721PixelContract::default().execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Sg721PixelContract::default().query(deps, env, msg)
    }
} 