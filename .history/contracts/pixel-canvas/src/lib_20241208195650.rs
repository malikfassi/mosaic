pub mod error;
pub mod state;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Addr, Coin,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::state::{Config, Pixel, CONFIG, PIXELS, OWNER_PIXELS};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:pixel-canvas";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Structs for messages
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub canvas_size: u32,  // Size of the canvas (N x N pixels)
    pub pixel_price: u128, // Initial price per pixel in ustars
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    BuyPixel { x: u32, y: u32 },
    SetPixelColor { x: u32, y: u32, color: String },
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPixel { x: u32, y: u32 },
    GetCanvas {},
    GetConfig {},
    GetOwnerPixels { owner: String },
}

// Contract entry points
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
        canvas_size: msg.canvas_size,
        pixel_price: msg.pixel_price,
    };
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("canvas_size", msg.canvas_size.to_string())
        .add_attribute("pixel_price", msg.pixel_price.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BuyPixel { x, y } => execute_buy_pixel(deps, env, info, x, y),
        ExecuteMsg::SetPixelColor { x, y, color } => execute_set_pixel_color(deps, env, info, x, y, color),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPixel { x, y } => to_binary(&query_pixel(deps, x, y)?),
        QueryMsg::GetCanvas {} => to_binary(&query_canvas(deps)?),
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetOwnerPixels { owner } => to_binary(&query_owner_pixels(deps, owner)?),
    }
}

// Contract execute functions
fn execute_buy_pixel(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    x: u32,
    y: u32,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // Validate coordinates
    if x >= config.canvas_size || y >= config.canvas_size {
        return Err(ContractError::InvalidPixelCoordinates { x, y });
    }
    
    // Check if pixel is already owned
    if PIXELS.may_load(deps.storage, (x, y))?.is_some() {
        return Err(ContractError::PixelAlreadyOwned {});
    }
    
    // Validate payment
    let payment = info
        .funds
        .iter()
        .find(|coin| coin.denom == "ustars")
        .ok_or(ContractError::InsufficientFunds {})?;
    
    if payment.amount.u128() < config.pixel_price {
        return Err(ContractError::InsufficientFunds {});
    }
    
    // Create new pixel
    let pixel = Pixel {
        owner: info.sender.clone(),
        color: "#FFFFFF".to_string(), // Default to white
        last_updated: env.block.time.seconds(),
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

fn execute_set_pixel_color(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    x: u32,
    y: u32,
    color: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // Validate coordinates
    if x >= config.canvas_size || y >= config.canvas_size {
        return Err(ContractError::InvalidPixelCoordinates { x, y });
    }
    
    // Validate color format (basic hex color validation)
    if !color.starts_with('#') || (color.len() != 7 && color.len() != 4) {
        return Err(ContractError::InvalidColorFormat {});
    }
    
    // Get existing pixel
    let mut pixel = PIXELS
        .may_load(deps.storage, (x, y))?
        .ok_or(ContractError::NotPixelOwner {})?;
    
    // Verify ownership
    if pixel.owner != info.sender {
        return Err(ContractError::NotPixelOwner {});
    }
    
    // Update pixel
    pixel.color = color.clone();
    pixel.last_updated = env.block.time.seconds();
    PIXELS.save(deps.storage, (x, y), &pixel)?;
    
    Ok(Response::new()
        .add_attribute("method", "set_pixel_color")
        .add_attribute("x", x.to_string())
        .add_attribute("y", y.to_string())
        .add_attribute("color", color))
}

// Query functions
fn query_pixel(deps: Deps, x: u32, y: u32) -> StdResult<Option<Pixel>> {
    PIXELS.may_load(deps.storage, (x, y))
}

fn query_canvas(deps: Deps) -> StdResult<Vec<(u32, u32, Pixel)>> {
    let config = CONFIG.load(deps.storage)?;
    let mut canvas = Vec::new();
    
    for x in 0..config.canvas_size {
        for y in 0..config.canvas_size {
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

fn query_owner_pixels(deps: Deps, owner: String) -> StdResult<Vec<(u32, u32)>> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    Ok(OWNER_PIXELS.may_load(deps.storage, owner_addr)?.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            canvas_size: 100,
            pixel_price: 1000000,
        };
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(3, res.attributes.len());
        
        // Test config query
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let config: Config = from_binary(&res).unwrap();
        assert_eq!(config.canvas_size, 100);
        assert_eq!(config.pixel_price, 1000000);
    }

    #[test]
    fn buy_pixel() {
        let mut deps = mock_dependencies();
        
        // Initialize contract
        let msg = InstantiateMsg {
            canvas_size: 100,
            pixel_price: 1000000,
        };
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        
        // Try to buy pixel with insufficient funds
        let info = mock_info("buyer", &coins(500000, "ustars"));
        let msg = ExecuteMsg::BuyPixel { x: 5, y: 5 };
        let err = execute(deps.as_mut(), mock_env(), info, msg.clone()).unwrap_err();
        match err {
            ContractError::InsufficientFunds {} => {}
            e => panic!("unexpected error: {}", e),
        }
        
        // Buy pixel successfully
        let info = mock_info("buyer", &coins(1000000, "ustars"));
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(3, res.attributes.len());
        
        // Query pixel
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetPixel { x: 5, y: 5 },
        ).unwrap();
        let pixel: Option<Pixel> = from_binary(&res).unwrap();
        assert!(pixel.is_some());
        let pixel = pixel.unwrap();
        assert_eq!(pixel.owner, Addr::unchecked("buyer"));
        assert_eq!(pixel.color, "#FFFFFF");
    }
} 