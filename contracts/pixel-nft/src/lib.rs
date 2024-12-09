pub mod error;
pub mod msg;
pub mod state;
#[cfg(test)]
mod testing;

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    SubMsg, WasmMsg, Addr, Reply, StdError,
};
use cw2::set_contract_version;
use sg721_pixel::msg::InstantiateMsg as Sg721InstantiateMsg;
use sg721_pixel::metadata::PixelMetadata;
use sg721_base::msg::CollectionInfo;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, COLORING_CONTRACT, NFT_CONTRACT};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pixel-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Reply IDs
const INSTANTIATE_NFT_REPLY_ID: u64 = 1;
const INSTANTIATE_COLORING_REPLY_ID: u64 = 2;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        admin: info.sender.clone(),
        canvas_width: msg.canvas_width,
        canvas_height: msg.canvas_height,
        pixel_price: msg.pixel_price,
        color_change_price: msg.color_change_price,
        color_change_cooldown: msg.color_change_cooldown,
    };
    CONFIG.save(deps.storage, &config)?;

    // Instantiate NFT contract
    let nft_msg = Sg721InstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        minter: env.contract.address.to_string(),
        collection_info: CollectionInfo {
            creator: info.sender.to_string(),
            description: "Mosaic Pixel NFTs".to_string(),
            image: msg.collection_image,
            external_link: None,
            royalty_info: None,
            explicit_content: None,
            start_trading_time: None,
        },
    };

    let instantiate_nft_msg = SubMsg::reply_on_success(
        WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id: msg.nft_code_id,
            msg: to_binary(&nft_msg)?,
            funds: vec![],
            label: "pixel_nft".to_string(),
        },
        INSTANTIATE_NFT_REPLY_ID,
    );

    Ok(Response::new()
        .add_submessage(instantiate_nft_msg)
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            pixel_price,
            color_change_price,
            color_change_cooldown,
        } => execute_update_config(deps, info, pixel_price, color_change_price, color_change_cooldown),
        ExecuteMsg::MintPixel { x, y, owner } => execute_mint_pixel(deps, info, x, y, owner),
    }
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    pixel_price: Option<u128>,
    color_change_price: Option<u128>,
    color_change_cooldown: Option<u64>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(price) = pixel_price {
        config.pixel_price = price.into();
    }
    if let Some(price) = color_change_price {
        config.color_change_price = price.into();
    }
    if let Some(cooldown) = color_change_cooldown {
        config.color_change_cooldown = cooldown;
    }

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("method", "update_config"))
}

pub fn execute_mint_pixel(
    deps: DepsMut,
    info: MessageInfo,
    x: u32,
    y: u32,
    owner: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let nft_contract = NFT_CONTRACT.load(deps.storage)?;

    // Check payment
    if info.funds.is_empty() || info.funds[0].amount < config.pixel_price {
        return Err(ContractError::InsufficientFunds {});
    }

    // Check coordinates
    if x >= config.canvas_width || y >= config.canvas_height {
        return Err(ContractError::InvalidCoordinates { x, y });
    }

    // Create mint message
    let token_id = format!("{}:{}", x, y);
    let metadata = PixelMetadata {
        x,
        y,
        color: "#FFFFFF".to_string(), // Default to white
    };
    let mint_msg = Sg721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: owner.clone(),
        token_uri: None,
        extension: metadata.into(),
    };

    let mint_msg = WasmMsg::Execute {
        contract_addr: nft_contract.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(mint_msg)
        .add_attribute("action", "mint_pixel")
        .add_attribute("token_id", token_id)
        .add_attribute("owner", owner)
        .add_attribute("x", x.to_string())
        .add_attribute("y", y.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::GetContracts {} => to_binary(&query_contracts(deps)?),
    }
}

fn query_contracts(deps: Deps) -> StdResult<(Option<Addr>, Option<Addr>)> {
    let nft = NFT_CONTRACT.may_load(deps.storage)?;
    let coloring = COLORING_CONTRACT.may_load(deps.storage)?;
    Ok((nft, coloring))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_NFT_REPLY_ID => handle_nft_instantiate_reply(deps, env, msg),
        INSTANTIATE_COLORING_REPLY_ID => handle_coloring_instantiate_reply(deps, msg),
        id => Err(ContractError::UnknownReplyId { id }),
    }
}

fn handle_nft_instantiate_reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let res = msg.result.into_result().map_err(|err| ContractError::Std(StdError::generic_err(err)))?;
    let event = res.events.iter().find(|e| e.ty == "instantiate").ok_or_else(|| {
        ContractError::Std(StdError::generic_err("cannot find instantiate event"))
    })?;
    let contract_addr = event
        .attributes
        .iter()
        .find(|attr| attr.key == "_contract_address")
        .ok_or_else(|| {
            ContractError::Std(StdError::generic_err("cannot find contract address"))
        })?
        .value
        .clone();

    let nft_addr = deps.api.addr_validate(&contract_addr)?;
    NFT_CONTRACT.save(deps.storage, &nft_addr)?;

    // Now instantiate the coloring contract
    let config = CONFIG.load(deps.storage)?;
    let coloring_msg = pixel_coloring::msg::InstantiateMsg {
        nft_contract: nft_addr.to_string(),
        price_per_color_change: config.color_change_price,
        color_change_cooldown: config.color_change_cooldown,
    };

    let instantiate_coloring_msg = SubMsg::reply_on_success(
        WasmMsg::Instantiate {
            admin: Some(config.admin.to_string()),
            code_id: config.coloring_code_id,
            msg: to_binary(&coloring_msg)?,
            funds: vec![],
            label: "pixel_coloring".to_string(),
        },
        INSTANTIATE_COLORING_REPLY_ID,
    );

    Ok(Response::new()
        .add_submessage(instantiate_coloring_msg)
        .add_attribute("nft_contract", nft_addr))
}

fn handle_coloring_instantiate_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    let res = msg.result.into_result().map_err(|err| ContractError::Std(StdError::generic_err(err)))?;
    let event = res.events.iter().find(|e| e.ty == "instantiate").ok_or_else(|| {
        ContractError::Std(StdError::generic_err("cannot find instantiate event"))
    })?;
    let contract_addr = event
        .attributes
        .iter()
        .find(|attr| attr.key == "_contract_address")
        .ok_or_else(|| {
            ContractError::Std(StdError::generic_err("cannot find contract address"))
        })?
        .value
        .clone();

    let coloring_addr = deps.api.addr_validate(&contract_addr)?;
    COLORING_CONTRACT.save(deps.storage, &coloring_addr)?;

    Ok(Response::new().add_attribute("coloring_contract", coloring_addr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{coins, from_json, Addr, OwnedDeps};
    use sg721_base::msg::CollectionInfoResponse;

    fn setup_test() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            name: "Pixel NFTs".to_string(),
            symbol: "PIXEL".to_string(),
            canvas_width: 1000,
            canvas_height: 1000,
            pixel_price: 1000000u128.into(),
            color_change_price: 500000u128.into(),
            color_change_cooldown: 3600,
            nft_code_id: 1,
            coloring_code_id: 2,
            collection_image: "ipfs://...".to_string(),
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        deps
    }

    #[test]
    fn proper_initialization() {
        let deps = setup_test();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let config: Config = from_json(&res).unwrap();
        assert_eq!(config.admin, Addr::unchecked("creator"));
        assert_eq!(config.canvas_width, 1000);
        assert_eq!(config.canvas_height, 1000);
        assert_eq!(config.pixel_price, 1000000u128.into());
        assert_eq!(config.color_change_price, 500000u128.into());
        assert_eq!(config.color_change_cooldown, 3600);
        assert_eq!(config.nft_code_id, 1);
        assert_eq!(config.coloring_code_id, 2);
    }

    #[test]
    fn mint_pixel() {
        let mut deps = setup_test();

        // Mock NFT contract address
        NFT_CONTRACT.save(deps.as_mut().storage, &Addr::unchecked("nft_contract")).unwrap();

        // Try to mint without payment
        let info = mock_info("buyer", &[]);
        let msg = ExecuteMsg::MintPixel {
            x: 0,
            y: 0,
            owner: "buyer".to_string(),
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        assert_eq!(err, ContractError::InsufficientFunds {});

        // Try to mint with correct payment
        let info = mock_info("buyer", &coins(1000000, "ustars"));
        let msg = ExecuteMsg::MintPixel {
            x: 0,
            y: 0,
            owner: "buyer".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Verify mint message
        let mint_msg = res.messages[0].msg.clone();
        match mint_msg {
            CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, msg, .. }) => {
                assert_eq!(contract_addr, "nft_contract");
                let decoded: Sg721ExecuteMsg = from_json(&msg).unwrap();
                match decoded {
                    Sg721ExecuteMsg::Mint { token_id, owner, .. } => {
                        assert_eq!(token_id, "0:0");
                        assert_eq!(owner, "buyer");
                    },
                    _ => panic!("unexpected message type"),
                }
            },
            _ => panic!("unexpected message type"),
        }
    }

    #[test]
    fn update_config() {
        let mut deps = setup_test();

        // Try to update with unauthorized user
        let info = mock_info("unauthorized", &[]);
        let msg = ExecuteMsg::UpdateConfig {
            pixel_price: Some(2000000),
            color_change_price: Some(1000000),
            color_change_cooldown: Some(7200),
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        // Update with authorized user
        let info = mock_info("creator", &[]);
        let msg = ExecuteMsg::UpdateConfig {
            pixel_price: Some(2000000),
            color_change_price: Some(1000000),
            color_change_cooldown: Some(7200),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Verify config update
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let config: Config = from_json(&res).unwrap();
        assert_eq!(config.pixel_price, 2000000u128.into());
        assert_eq!(config.color_change_price, 1000000u128.into());
        assert_eq!(config.color_change_cooldown, 7200);
    }
} 