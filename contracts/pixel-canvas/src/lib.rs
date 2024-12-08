pub mod error;
pub mod msg;
pub mod state;

use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Pixel, PIXELS, CONFIG, OWNER_PIXELS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pixel-canvas";
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
        width: msg.width,
        height: msg.height,
        price_per_pixel: msg.price_per_pixel,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("width", msg.width.to_string())
        .add_attribute("height", msg.height.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BuyPixel { x, y } => execute_buy_pixel(deps, info, x, y),
        ExecuteMsg::SetPixelColor { x, y, color } => execute_set_pixel_color(deps, info, x, y, color),
    }
}

pub fn execute_buy_pixel(
    deps: DepsMut,
    info: MessageInfo,
    x: u32,
    y: u32,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Check coordinates are within bounds
    if x >= config.width || y >= config.height {
        return Err(ContractError::InvalidCoordinates { x, y });
    }

    // Check if pixel is already owned
    if PIXELS.may_load(deps.storage, (x, y))?.is_some() {
        return Err(ContractError::PixelAlreadyOwned {});
    }

    // Check payment
    if info.funds.is_empty() || info.funds[0].amount < config.price_per_pixel {
        return Err(ContractError::InsufficientFunds {});
    }

    // Create new pixel
    let pixel = Pixel {
        owner: info.sender.clone(),
        color: "#FFFFFF".to_string(), // Default to white
    };

    // Save pixel
    PIXELS.save(deps.storage, (x, y), &pixel)?;

    // Update owner's pixels
    let mut owner_pixels = OWNER_PIXELS
        .may_load(deps.storage, info.sender.clone())?
        .unwrap_or_default();
    owner_pixels.push((x, y));
    OWNER_PIXELS.save(deps.storage, info.sender.clone(), &owner_pixels)?;

    Ok(Response::new()
        .add_attribute("method", "buy_pixel")
        .add_attribute("owner", info.sender)
        .add_attribute("x", x.to_string())
        .add_attribute("y", y.to_string()))
}

pub fn execute_set_pixel_color(
    deps: DepsMut,
    info: MessageInfo,
    x: u32,
    y: u32,
    color: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Check coordinates are within bounds
    if x >= config.width || y >= config.height {
        return Err(ContractError::InvalidCoordinates { x, y });
    }

    // Check color format (should be a valid hex color)
    if !color.starts_with('#') || color.len() != 7 {
        return Err(ContractError::InvalidColorFormat {});
    }

    // Get pixel and verify ownership
    let mut pixel = PIXELS
        .may_load(deps.storage, (x, y))?
        .ok_or(ContractError::Unauthorized {})?;

    if pixel.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // Update pixel color
    pixel.color = color;
    PIXELS.save(deps.storage, (x, y), &pixel)?;

    Ok(Response::new()
        .add_attribute("method", "set_pixel_color")
        .add_attribute("owner", info.sender)
        .add_attribute("x", x.to_string())
        .add_attribute("y", y.to_string())
        .add_attribute("color", pixel.color))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPixel { x, y } => to_json_binary(&query_pixel(deps, x, y)?),
        QueryMsg::GetCanvas {} => to_json_binary(&query_canvas(deps)?),
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetOwnerPixels { owner } => to_json_binary(&query_owner_pixels(deps, owner)?),
    }
}

fn query_pixel(deps: Deps, x: u32, y: u32) -> StdResult<Option<Pixel>> {
    PIXELS.may_load(deps.storage, (x, y))
}

fn query_canvas(deps: Deps) -> StdResult<Vec<(u32, u32, Pixel)>> {
    let config = CONFIG.load(deps.storage)?;
    let mut canvas = Vec::new();

    for x in 0..config.width {
        for y in 0..config.height {
            if let Some(pixel) = PIXELS.may_load(deps.storage, (x, y))? {
                canvas.push((x, y, pixel));
            }
        }
    }

    Ok(canvas)
}

fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

fn query_owner_pixels(deps: Deps, owner: String) -> StdResult<Vec<(u32, u32, Pixel)>> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let owner_pixels = OWNER_PIXELS.may_load(deps.storage, owner_addr)?.unwrap_or_default();
    let mut pixels = Vec::new();

    for (x, y) in owner_pixels {
        if let Some(pixel) = PIXELS.may_load(deps.storage, (x, y))? {
            pixels.push((x, y, pixel));
        }
    }

    Ok(pixels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, Addr, Uint128};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            width: 100,
            height: 100,
            price_per_pixel: Uint128::from(1000000u128),
        };
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let config: Config = from_json(&res).unwrap();
        assert_eq!(config.width, 100);
        assert_eq!(config.height, 100);
        assert_eq!(config.price_per_pixel, Uint128::from(1000000u128));
    }

    #[test]
    fn buy_pixel() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            width: 100,
            height: 100,
            price_per_pixel: Uint128::from(1000000u128),
        };
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Buy a pixel
        let info = mock_info("buyer", &coins(1000000, "ustars"));
        let msg = ExecuteMsg::BuyPixel { x: 0, y: 0 };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Query the pixel
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPixel { x: 0, y: 0 }).unwrap();
        let pixel: Option<Pixel> = from_json(&res).unwrap();
        let pixel = pixel.unwrap();
        assert_eq!(pixel.owner, Addr::unchecked("buyer"));
        assert_eq!(pixel.color, "#FFFFFF");
    }
} 