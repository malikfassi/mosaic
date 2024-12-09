pub mod error;
pub mod msg;
pub mod state;

use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, WasmQuery, QueryRequest,
};
use cw2::set_contract_version;
use sg721_pixel::{PixelExtension, QueryMsg as Sg721QueryMsg};

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
        ExecuteMsg::SetPixelColor { x, y, color } => {
            execute_set_pixel_color(deps, env, info, x, y, color)
        }
        ExecuteMsg::UpdateConfig {
            price_per_color_change,
            color_change_cooldown,
        } => execute_update_config(deps, info, price_per_color_change, color_change_cooldown),
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
    let config = CONFIG.load(deps.storage)?;

    // Validate color format
    if !color.starts_with('#') || color.len() != 7 {
        return Err(ContractError::InvalidColorFormat {});
    }

    // Check payment
    if info.funds.is_empty() || info.funds[0].amount < config.price_per_color_change {
        return Err(ContractError::InsufficientFunds {});
    }

    // Query NFT ownership
    let nft_query = Sg721QueryMsg::OwnerOf {
        token_id: format!("{}_{}", x, y),
        include_expired: None,
    };
    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.nft_contract.to_string(),
        msg: to_json_binary(&nft_query)?,
    });
    let owner_response: StdResult<sg721_pixel::OwnerOfResponse> = deps.querier.query(&query_req);

    match owner_response {
        Ok(response) => {
            if response.owner != info.sender {
                return Err(ContractError::NotPixelOwner {});
            }
        }
        Err(_) => return Err(ContractError::PixelNotFound {}),
    }

    // Check cooldown
    if let Some(last_change) = COLOR_CHANGES.may_load(deps.storage, (x, y))? {
        if env.block.time.seconds() < last_change.last_change + config.color_change_cooldown {
            return Err(ContractError::ColorChangeCooldown {});
        }
    }

    // Save color change
    let color_change = ColorChange {
        last_change: env.block.time.seconds(),
        color: color.clone(),
    };
    COLOR_CHANGES.save(deps.storage, (x, y), &color_change)?;

    Ok(Response::new()
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

    // Check if sender is owner
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    // Update config
    if let Some(price) = price_per_color_change {
        config.price_per_color_change = price;
    }
    if let Some(cooldown) = color_change_cooldown {
        config.color_change_cooldown = cooldown;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::GetPixelColor { x, y } => to_json_binary(&COLOR_CHANGES.may_load(deps.storage, (x, y))?),
        QueryMsg::GetPixelColors { start_after, limit } => {
            let limit = limit.unwrap_or(30) as usize;
            let start = start_after.map(|coords| COLOR_CHANGES.key((coords.0, coords.1)));

            let colors: StdResult<Vec<_>> = COLOR_CHANGES
                .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
                .take(limit)
                .map(|item| {
                    let ((x, y), color_change) = item?;
                    Ok((x, y, color_change))
                })
                .collect();

            to_json_binary(&colors?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, Addr};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            nft_contract: "nft_contract".to_string(),
            price_per_color_change: Uint128::from(1000000u128),
            color_change_cooldown: 3600,
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let config: Config = from_json(&res).unwrap();
        assert_eq!(config.owner, Addr::unchecked("creator"));
        assert_eq!(config.nft_contract, Addr::unchecked("nft_contract"));
        assert_eq!(config.price_per_color_change, Uint128::from(1000000u128));
        assert_eq!(config.color_change_cooldown, 3600);
    }
} 