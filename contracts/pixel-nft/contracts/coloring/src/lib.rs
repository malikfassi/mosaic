pub mod error;
pub mod msg;
pub mod state;

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, CosmosMsg, WasmMsg,
};
use cw2::set_contract_version;
use sg721_base::msg::QueryMsg as Sg721QueryMsg;
use sg721_base::msg::ExecuteMsg as Sg721ExecuteMsg;
use sg_metadata::Metadata;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, ColorChange, CONFIG, COLOR_CHANGES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pixel-coloring";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: info.sender.clone(),
        nft_contract: deps.api.addr_validate(&msg.nft_contract)?,
        price_per_color_change: msg.price_per_color_change,
        color_change_cooldown: msg.color_change_cooldown,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("nft_contract", msg.nft_contract))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPixelColor { x, y, color } => execute_set_pixel_color(deps, env, info, x, y, color),
        ExecuteMsg::UpdateConfig { price_per_color_change, color_change_cooldown } => {
            execute_update_config(deps, info, price_per_color_change, color_change_cooldown)
        },
    }
}

pub fn execute_set_pixel_color(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    x: u32,
    y: u32,
    color: String,
) -> Result<Response, ContractError> {
    // Check color format (should be a valid hex color)
    if !color.starts_with('#') || color.len() != 7 {
        return Err(ContractError::InvalidColorFormat {});
    }

    let config = CONFIG.load(deps.storage)?;

    // Check payment
    if info.funds.is_empty() || info.funds[0].amount < config.price_per_color_change {
        return Err(ContractError::InsufficientFunds {});
    }

    // Check cooldown
    if let Some(last_change) = COLOR_CHANGES.may_load(deps.storage, (x, y))? {
        if env.block.time.seconds() < last_change.last_change + config.color_change_cooldown {
            return Err(ContractError::ColorChangeCooldown {});
        }
    }

    // Query NFT ownership
    let token_id = format!("{}:{}", x, y);
    let owner_query = Sg721QueryMsg::OwnerOf { 
        token_id: token_id.clone(),
        include_expired: None,
    };
    let owner_response: StdResult<sg721_base::msg::OwnerOfResponse> = deps.querier.query_wasm_smart(
        config.nft_contract.clone(),
        &owner_query,
    );

    match owner_response {
        Ok(response) => {
            if response.owner != info.sender {
                return Err(ContractError::NotPixelOwner {});
            }
        },
        Err(_) => return Err(ContractError::PixelNotFound {}),
    }

    // Update color
    let color_change = ColorChange {
        last_change: env.block.time.seconds(),
        color: color.clone(),
    };
    COLOR_CHANGES.save(deps.storage, (x, y), &color_change)?;

    // Create message to update NFT metadata
    let metadata = sg_metadata::Metadata {
        image: None,
        image_data: None,
        external_url: None,
        description: Some(format!("Pixel at ({}, {})", x, y)),
        name: Some(format!("Pixel ({}, {})", x, y)),
        attributes: Some(vec![
            sg_metadata::Trait {
                display_type: None,
                trait_type: "x".to_string(),
                value: x.to_string(),
            },
            sg_metadata::Trait {
                display_type: None,
                trait_type: "y".to_string(),
                value: y.to_string(),
            },
            sg_metadata::Trait {
                display_type: None,
                trait_type: "color".to_string(),
                value: color.clone(),
            },
        ]),
        background_color: None,
        animation_url: None,
        youtube_url: None,
    };

    let update_msg = Sg721ExecuteMsg::UpdateMetadata { 
        token_id,
        token_uri: None,
        extension: Some(metadata),
    };

    let update_metadata_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.nft_contract.to_string(),
        msg: to_binary(&update_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(update_metadata_msg)
        .add_attribute("method", "set_pixel_color")
        .add_attribute("owner", info.sender)
        .add_attribute("x", x.to_string())
        .add_attribute("y", y.to_string())
        .add_attribute("color", color))
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    price_per_color_change: Option<Uint128>,
    color_change_cooldown: Option<u64>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Only owner can update config
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(price) = price_per_color_change {
        config.price_per_color_change = price;
    }

    if let Some(cooldown) = color_change_cooldown {
        config.color_change_cooldown = cooldown;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "update_config")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetPixelColor { x, y } => to_json_binary(&query_pixel_color(deps, x, y)?),
        QueryMsg::GetPixelColors { start_after, limit } => 
            to_json_binary(&query_pixel_colors(deps, start_after, limit)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

fn query_pixel_color(deps: Deps, x: u32, y: u32) -> StdResult<Option<ColorChange>> {
    COLOR_CHANGES.may_load(deps.storage, (x, y))
}

fn query_pixel_colors(
    deps: Deps,
    start_after: Option<(u32, u32)>,
    limit: Option<u32>,
) -> StdResult<Vec<(u32, u32, ColorChange)>> {
    let limit = limit.unwrap_or(30) as usize;
    let start = start_after.map(|coords| coords);

    let colors: StdResult<Vec<_>> = COLOR_CHANGES
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit)
        .map(|item| {
            let ((x, y), color_change) = item?;
            Ok((x, y, color_change))
        })
        .collect();

    colors
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{coins, from_json, Addr, OwnedDeps, to_json_binary};

    fn setup_test() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            nft_contract: "nft_contract".to_string(),
            price_per_color_change: 500000u128.into(),
            color_change_cooldown: 3600,
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Mock NFT contract queries
        deps.querier.update_wasm(|query| {
            match query {
                cosmwasm_std::WasmQuery::Smart { contract_addr, msg } => {
                    if contract_addr == "nft_contract" {
                        let query_msg: Sg721QueryMsg = from_json(msg).unwrap();
                        match query_msg {
                            Sg721QueryMsg::OwnerOf { token_id, .. } => {
                                let owner = if token_id == "0:0" {
                                    "owner1"
                                } else if token_id == "1:1" {
                                    "owner2"
                                } else {
                                    return Err(StdError::not_found("token not found").into());
                                };
                                Ok(to_json_binary(&sg721_base::msg::OwnerOfResponse {
                                    owner: Addr::unchecked(owner),
                                    approvals: vec![],
                                }).unwrap())
                            },
                            _ => Err(StdError::not_found("unsupported query").into()),
                        }
                    } else {
                        Err(StdError::not_found("contract not found").into())
                    }
                },
                _ => Err(StdError::not_found("unsupported query").into()),
            }
        });

        deps
    }

    #[test]
    fn proper_initialization() {
        let deps = setup_test();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let config: Config = from_json(&res).unwrap();
        assert_eq!(config.owner, Addr::unchecked("creator"));
        assert_eq!(config.nft_contract, Addr::unchecked("nft_contract"));
        assert_eq!(config.price_per_color_change, 500000u128.into());
        assert_eq!(config.color_change_cooldown, 3600);
    }

    #[test]
    fn set_pixel_color() {
        let mut deps = setup_test();

        // Try to set color without payment
        let info = mock_info("owner1", &[]);
        let msg = ExecuteMsg::SetPixelColor {
            x: 0,
            y: 0,
            color: "#FF0000".to_string(),
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        assert_eq!(err, ContractError::InsufficientFunds {});

        // Try to set color for unowned pixel
        let info = mock_info("not_owner", &coins(500000, "ustars"));
        let msg = ExecuteMsg::SetPixelColor {
            x: 0,
            y: 0,
            color: "#FF0000".to_string(),
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        assert_eq!(err, ContractError::NotPixelOwner {});

        // Set color successfully
        let info = mock_info("owner1", &coins(500000, "ustars"));
        let msg = ExecuteMsg::SetPixelColor {
            x: 0,
            y: 0,
            color: "#FF0000".to_string(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(1, res.messages.len());

        // Verify color change message
        let color_msg = res.messages[0].msg.clone();
        match color_msg {
            CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, msg, .. }) => {
                assert_eq!(contract_addr, "nft_contract");
                let decoded: Sg721ExecuteMsg = from_json(&msg).unwrap();
                match decoded {
                    Sg721ExecuteMsg::UpdateMetadata { token_id, extension, .. } => {
                        assert_eq!(token_id, "0:0");
                        let metadata = extension.unwrap();
                        assert_eq!(metadata.attributes.unwrap()[2].value, "#FF0000");
                    },
                    _ => panic!("unexpected message type"),
                }
            },
            _ => panic!("unexpected message type"),
        }

        // Query color change
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPixelColor { x: 0, y: 0 }).unwrap();
        let color_change: Option<ColorChange> = from_json(&res).unwrap();
        assert!(color_change.is_some());
        assert_eq!(color_change.unwrap().color, "#FF0000");
    }

    #[test]
    fn update_config() {
        let mut deps = setup_test();

        // Try to update with unauthorized user
        let info = mock_info("unauthorized", &[]);
        let msg = ExecuteMsg::UpdateConfig {
            price_per_color_change: Some(1000000u128.into()),
            color_change_cooldown: Some(7200),
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        // Update with authorized user
        let info = mock_info("creator", &[]);
        let msg = ExecuteMsg::UpdateConfig {
            price_per_color_change: Some(1000000u128.into()),
            color_change_cooldown: Some(7200),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Verify config update
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let config: Config = from_json(&res).unwrap();
        assert_eq!(config.price_per_color_change, 1000000u128.into());
        assert_eq!(config.color_change_cooldown, 7200);
    }
} 