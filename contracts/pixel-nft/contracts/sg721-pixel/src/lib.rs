pub mod metadata;
pub mod msg;
pub mod state;

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use sg721_base::{ContractError, InstantiateMsg, QueryMsg, SudoMsg};
use sg_metadata::Metadata;

use crate::msg::ExecuteMsg;
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sg721-pixel";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // First, instantiate the base contract
    let res = sg721_base::contract::instantiate(deps.branch(), env.clone(), info.clone(), msg.clone())?;

    // Then, set up our custom config
    let config = Config {
        canvas_width: msg.canvas_width,
        canvas_height: msg.canvas_height,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Base(base_msg) => sg721_base::contract::execute(deps, env, info, base_msg),
        ExecuteMsg::UpdateMetadata { token_id, token_uri, extension } => {
            // Only minter can update metadata
            let minter = sg721_base::state::MINTER.load(deps.storage)?;
            if info.sender != minter {
                return Err(ContractError::Unauthorized {});
            }

            // Update metadata
            let mut token = sg721_base::state::TokenInfo::<Metadata>::load(deps.storage, &token_id)?;
            if let Some(token_uri) = token_uri {
                token.token_uri = Some(token_uri);
            }
            if let Some(extension) = extension {
                token.extension = extension;
            }
            token.save(deps.storage)?;

            Ok(Response::new()
                .add_attribute("action", "update_metadata")
                .add_attribute("token_id", token_id))
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    sg721_base::contract::query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    sg721_base::contract::sudo(deps, env, msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, Addr};
    use sg721_base::msg::CollectionInfoResponse;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            name: "Pixel NFTs".to_string(),
            symbol: "PIXEL".to_string(),
            minter: "creator".to_string(),
            collection_info: sg721::CollectionInfo {
                creator: "creator".to_string(),
                description: "Pixel NFTs".to_string(),
                image: "ipfs://...".to_string(),
                external_link: None,
                royalty_info: None,
                explicit_content: None,
                start_trading_time: None,
            },
            canvas_width: 1000,
            canvas_height: 1000,
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::CollectionInfo {}).unwrap();
        let collection_info: CollectionInfoResponse = from_json(&res).unwrap();
        assert_eq!(collection_info.creator, Addr::unchecked("creator"));
    }
} 