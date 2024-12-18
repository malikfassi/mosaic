pub mod contract;
mod error;
pub mod msg;
mod state;

pub use crate::error::ContractError;
pub use crate::state::Sg721Contract;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use cw721_base::Extension;
use msg::CustomExecuteMsg;

pub type ExecuteMsg = sg721::ExecuteMsg<Extension, CustomExecuteMsg>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

pub mod entry {
    use super::*;
    use crate::{msg::QueryMsg, state::Sg721Contract};

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw2::set_contract_version;
    use sg721::InstantiateMsg;

    // version info for migration info
    pub const CONTRACT_NAME: &str = "crates.io:mosaic-nft";
    pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let res = Sg721Contract::<Extension>::default().instantiate(deps, env, info, msg)?;

        Ok(res
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let contract = Sg721Contract::<Extension>::default();
        contract.execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Sg721Contract::<Extension>::default().query(deps, env, msg)
    }
}

#[cw_serde]
pub struct TileExtension {
    pub pixels: Vec<Pixel>,
}

#[cw_serde]
pub struct Pixel {
    pub color: [u8; 3],
    pub expiration: u64,
}

#[cfg(test)]
mod tests; 