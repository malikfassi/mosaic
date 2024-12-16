use cosmwasm_std::{
    to_json_binary, BankMsg, Coin, DepsMut, Empty, Env, MessageInfo, Response, Uint128, WasmMsg,
};
use cw721_base::{msg::ExecuteMsg as Cw721ExecuteMsg, state::TokenInfo};
use std::collections::HashMap;

use crate::{
    error::ContractError,
    msg::PixelUpdate,
    state::{
        get_tile_id_from_pixel, validate_pixel_id, validate_tile_id, Color, Cw721StorageType,
        TileMetadata, DEVELOPER, DEVELOPER_FEE, MINTER, OWNER_FEE, PIXEL_COLORS, TOKEN_COUNT,
    },
};

// Constants
const MAX_BATCH_SIZE: u32 = 100;

pub fn execute_mint_tile(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    tile_id: u32,
    owner: String,
) -> Result<Response, ContractError> {
    // Validate minter
    let minter = MINTER.load(deps.storage)?;
    if info.sender != minter {
        return Err(ContractError::Unauthorized {});
    }

    // Validate tile ID
    if !validate_tile_id(tile_id) {
        return Err(ContractError::InvalidTileId { tile_id });
    }

    // Convert tile_id to token_id
    let token_id = tile_id.to_string();

    // Create token info
    let token = TokenInfo {
        owner: deps.api.addr_validate(&owner)?,
        approvals: vec![],
        token_uri: None,
        extension: TileMetadata::default(),
    };

    // Save token info
    let contract = Cw721StorageType::default();
    contract.tokens.save(deps.storage, &token_id, &token)?;

    // Increment token count
    let mut count = TOKEN_COUNT.load(deps.storage)?;
    count += 1;
    TOKEN_COUNT.save(deps.storage, &count)?;

    // Create mint message
    let mint_msg: Cw721ExecuteMsg<TileMetadata, Empty> = Cw721ExecuteMsg::Mint {
        token_id: token_id.clone(),
        owner: owner.clone(),
        token_uri: None,
        extension: TileMetadata::default(),
    };

    let msg = WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_json_binary(&mint_msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "mint_tile")
        .add_attribute("tile_id", tile_id.to_string())
        .add_attribute("owner", owner)
        .add_attribute("token_count", count.to_string()))
}

pub fn execute_set_pixel_color(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    pixel_id: u32,
    color: Color,
) -> Result<Response, ContractError> {
    // Validate pixel ID
    if !validate_pixel_id(pixel_id) {
        return Err(ContractError::InvalidPixelId { pixel_id });
    }

    // Get tile ID
    let tile_id = get_tile_id_from_pixel(pixel_id)
        .ok_or(ContractError::InvalidPixelId { pixel_id })?;

    // Check if tile exists
    let contract = Cw721StorageType::default();
    let token = contract
        .tokens
        .load(deps.storage, &tile_id.to_string())
        .map_err(|_| ContractError::InvalidPixelId { pixel_id })?;

    // Validate fees
    let developer_fee = DEVELOPER_FEE.load(deps.storage)?;
    let owner_fee = OWNER_FEE.load(deps.storage)?;
    let required_funds = vec![developer_fee.clone(), owner_fee.clone()];

    if !has_sufficient_funds(&info.funds, &required_funds) {
        return Err(ContractError::InsufficientFunds {});
    }

    let developer = DEVELOPER.load(deps.storage)?;

    // Update the color in storage
    PIXEL_COLORS.save(deps.storage, pixel_id, &color.pack())?;

    // Send fees
    Ok(Response::new()
        .add_message(BankMsg::Send {
            to_address: developer,
            amount: vec![developer_fee],
        })
        .add_message(BankMsg::Send {
            to_address: token.owner.to_string(),
            amount: vec![owner_fee],
        })
        .add_attribute("action", "set_pixel_color")
        .add_attribute("pixel_id", pixel_id.to_string())
        .add_attribute("tile_id", tile_id.to_string())
        .add_attribute("color", format!("{:?}", color)))
}

pub fn execute_batch_set_pixels(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    updates: Vec<PixelUpdate>,
) -> Result<Response, ContractError> {
    // Validate batch size
    if updates.len() > MAX_BATCH_SIZE as usize {
        return Err(ContractError::BatchTooLarge {
            max: MAX_BATCH_SIZE,
        });
    }

    // Calculate total fees
    let developer_fee = DEVELOPER_FEE.load(deps.storage)?;
    let owner_fee = OWNER_FEE.load(deps.storage)?;
    let total_developer_fee = multiply_coin(&developer_fee, updates.len() as u128);
    let total_owner_fee = multiply_coin(&owner_fee, updates.len() as u128);
    let required_funds = vec![total_developer_fee.clone(), total_owner_fee.clone()];

    if !has_sufficient_funds(&info.funds, &required_funds) {
        return Err(ContractError::InsufficientFunds {});
    }

    // Group updates by tile for fee calculation
    let mut tile_owners: HashMap<u32, String> = HashMap::new();
    let mut updates_by_tile: HashMap<u32, Vec<(u32, Color)>> = HashMap::new();

    for update in &updates {
        // Validate pixel ID
        if !validate_pixel_id(update.pixel_id) {
            return Err(ContractError::InvalidPixelId {
                pixel_id: update.pixel_id,
            });
        }

        let tile_id = get_tile_id_from_pixel(update.pixel_id).ok_or({
            ContractError::InvalidPixelId {
                pixel_id: update.pixel_id,
            }
        })?;

        // Get tile owner if we haven't yet
        if let std::collections::hash_map::Entry::Vacant(e) = tile_owners.entry(tile_id) {
            let contract = Cw721StorageType::default();
            let token = contract.tokens.load(deps.storage, &tile_id.to_string())?;
            e.insert(token.owner.to_string());
        }

        updates_by_tile
            .entry(tile_id)
            .or_default()
            .push((update.pixel_id, update.color.clone()));
    }

    // Apply all updates
    for (_, pixel_updates) in updates_by_tile {
        for (pixel_id, color) in pixel_updates {
            PIXEL_COLORS.save(deps.storage, pixel_id, &color.pack())?;
        }
    }

    // Send fees
    let developer = DEVELOPER.load(deps.storage)?;
    let mut messages = vec![];

    if !total_developer_fee.amount.is_zero() {
        messages.push(BankMsg::Send {
            to_address: developer,
            amount: vec![total_developer_fee],
        });
    }

    // Send fees to each tile owner proportionally
    for (tile_id, owner) in tile_owners {
        let tile_updates = updates
            .iter()
            .filter(|u| get_tile_id_from_pixel(u.pixel_id) == Some(tile_id))
            .count() as u128;

        let owner_fee_amount = multiply_coin(&owner_fee, tile_updates);
        if !owner_fee_amount.amount.is_zero() {
            messages.push(BankMsg::Send {
                to_address: owner,
                amount: vec![owner_fee_amount],
            });
        }
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "batch_set_pixels")
        .add_attribute("update_count", updates.len().to_string()))
}

// Helper function to check if provided funds are sufficient
fn has_sufficient_funds(provided: &[Coin], required: &[Coin]) -> bool {
    for req in required {
        let provided_amount = provided
            .iter()
            .find(|c| c.denom == req.denom)
            .map(|c| c.amount)
            .unwrap_or_default();
        if provided_amount < req.amount {
            return false;
        }
    }
    true
}

// Helper function to multiply a coin amount
fn multiply_coin(coin: &Coin, multiplier: u128) -> Coin {
    Coin {
        denom: coin.denom.clone(),
        amount: coin.amount * Uint128::from(multiplier),
    }
}
