pub mod contract;
pub mod error;
pub mod msg;
pub mod types;
pub mod constants;

use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, from_json};
use sg721_base::{self, msg::QueryMsg};
use sg721::InstantiateMsg;
use cw721_base::MinterResponse;
use sg_std::StargazeMsgWrapper;

use crate::{
    error::ContractError,
    msg::ExecuteMsg,
    contract::execute_set_pixel_color,
    types::Extension,
};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:mosaic-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct MosaicContract<'a> {
    pub base: sg721_base::Sg721Contract<'a, Extension>,
}

impl<'a> Default for MosaicContract<'a> {
    fn default() -> Self {
        Self {
            base: sg721_base::Sg721Contract::default(),
        }
    }
}

impl<'a> MosaicContract<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response<StargazeMsgWrapper>, ContractError> {
        // Set contract version first
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        
        // Initialize base contract
        let res = self.base.instantiate(deps, env, info, msg)
            .map_err(|e| ContractError::Base(e.to_string()))?;

        Ok(Response::new()
            .add_attributes(res.attributes)
            .add_events(res.events))
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response<StargazeMsgWrapper>, ContractError> {
        match msg {
            // Forward all base NFT functionality to base contract
            ExecuteMsg::Base(base_msg) => {
                self.base.execute(deps, env, info, base_msg)
                    .map_err(|e| ContractError::Base(e.to_string()))
            }
            // Custom pixel color functionality
            ExecuteMsg::SetPixelColor { current_tile_metadata, pixel_update } => {
                // Verify sender is the minter
                let minter: MinterResponse = from_json(
                    &self.base.query(deps.as_ref(), env.clone(), QueryMsg::Minter {})
                        .map_err(|e| ContractError::Std(StdError::generic_err(e.to_string())))?
                )?;
                
                if info.sender.to_string() != minter.minter.unwrap_or_default() {
                    return Err(ContractError::Unauthorized {});
                }
                execute_set_pixel_color(deps, env, info, current_tile_metadata, pixel_update)
            }
        }
    }

    pub fn query(
        &self,
        deps: Deps,
        env: Env,
        msg: QueryMsg,
    ) -> Result<Binary, ContractError> {
        // Forward all queries to base contract
        self.base.query(deps, env, msg)
            .map_err(|e| ContractError::Std(StdError::generic_err(e.to_string())))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract = MosaicContract::default();
    contract.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    let contract = MosaicContract::default();
    contract.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    let contract = MosaicContract::default();
    contract.query(deps, env, msg)
}

#[cfg(test)]
mod tests;