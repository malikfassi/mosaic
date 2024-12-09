pub mod metadata;
pub mod msg;
pub mod state;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Empty,
};
use sg721::{
    entry::{execute as base_execute, instantiate as base_instantiate, query as base_query},
    msg::{ExecuteMsg as Sg721ExecuteMsg, QueryMsg as Sg721QueryMsg, CollectionInfoResponse},
    ContractError,
};
use sg_metadata::Metadata;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let res = base_instantiate(deps, env, info, msg)?;
    Ok(Response::new()
        .add_attributes(res.attributes))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Base(base_msg) => {
            let res = base_execute(deps, env, info, base_msg)?;
            Ok(Response::new()
                .add_attributes(res.attributes))
        }
        ExecuteMsg::UpdateMetadata { token_id, token_uri, extension } => {
            // Only minter can update metadata
            let minter = deps.querier.query_wasm_smart(
                env.contract.address,
                &Sg721QueryMsg::Minter {},
            )?;
            if info.sender != minter {
                return Err(ContractError::Unauthorized {});
            }

            // Update metadata
            let mut token = deps.querier.query_wasm_smart(
                env.contract.address,
                &Sg721QueryMsg::NftInfo { token_id: token_id.clone() },
            )?;
            if let Some(token_uri) = token_uri {
                token.token_uri = Some(token_uri);
            }
            if let Some(extension) = extension {
                token.extension = extension;
            }

            // Save token
            let update_msg = Sg721ExecuteMsg::Extension {
                msg: Empty {},
            };
            let res = base_execute(deps, env, info, update_msg)?;

            Ok(Response::new()
                .add_attributes(res.attributes)
                .add_attribute("action", "update_metadata")
                .add_attribute("token_id", token_id))
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    base_query(deps, env, msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::from_json;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            name: "Pixel NFT".to_string(),
            symbol: "PIXEL".to_string(),
            minter: "creator".to_string(),
            collection_info: CollectionInfoResponse {
                creator: "creator".to_string(),
                description: "Pixel NFT Collection".to_string(),
                image: "https://example.com/image.png".to_string(),
                external_link: None,
                explicit_content: None,
                start_trading_time: None,
                royalty_info: None,
            },
        };

        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());

        // Check collection info
        let res = query(deps.as_ref(), mock_env(), QueryMsg::CollectionInfo {}).unwrap();
        let collection_info: CollectionInfoResponse = from_json(&res).unwrap();
        assert_eq!(collection_info.creator, "creator");
        assert_eq!(collection_info.description, "Pixel NFT Collection");
        assert_eq!(collection_info.image, "https://example.com/image.png");
    }
} 