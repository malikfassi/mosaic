use crate::state::Sg721PixelContract;
use cosmwasm_std::{DepsMut, Env, Response};
use cw2::set_contract_version;

pub const CONTRACT_NAME: &str = "crates.io:sg721-pixel";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> Sg721PixelContract {
    pub fn migrate(
        &self,
        deps: DepsMut,
        _env: Env,
        _contract_version: &str,
    ) -> Result<Response, crate::ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::new()
            .add_attribute("action", "migrate")
            .add_attribute("from_version", _contract_version)
            .add_attribute("to_version", CONTRACT_VERSION))
    }
} 