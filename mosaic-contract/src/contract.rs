use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, BankMsg, Coin, to_json_binary, from_json, Empty};
use sg_std::StargazeMsgWrapper;
use cw721::OwnerOfResponse;
use crate::{
    error::ContractError,
    msg::PixelUpdate,
    types::TileMetadata,
    constants::fees,
};

pub fn execute_set_pixel_color(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    current_tile_metadata: Vec<u8>,
    pixel_update: PixelUpdate,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    // Calculate expiration duration and required fee
    let duration = pixel_update.expiration.saturating_sub(env.block.time.seconds());
    let required_fee = fees::calculate_fee(duration);

    // Validate payment
    if info.funds.len() != 1 || info.funds[0] != required_fee {
        return Err(ContractError::InvalidFee {
            expected: required_fee,
            received: info.funds.get(0).cloned(),
        });
    }

    // Parse the current tile metadata
    let tile_metadata = TileMetadata::from_bytes(&current_tile_metadata)
        .map_err(|e| ContractError::InvalidPixelUpdate(e.to_string()))?;

    // Verify pixel is in tile range
    if !crate::types::is_pixel_in_tile(pixel_update.pixel_id, tile_metadata.tile_id) {
        return Err(ContractError::PixelOutOfRange {});
    }

    // Get current pixel metadata
    let current_pixel = tile_metadata.get_pixel(pixel_update.pixel_id)
        .map_err(|e| ContractError::InvalidPixelUpdate(e))?;

    // Verify pixel is not expired
    if current_pixel.expiration > env.block.time.seconds() {
        return Err(ContractError::InvalidPixelUpdate("Pixel is currently locked".to_string()));
    }

    // Get the owner of the tile (NFT)
    let owner_response: OwnerOfResponse = deps.querier.query_wasm_smart(
        info.sender.clone(),
        &cw721::Cw721QueryMsg::OwnerOf {
            token_id: tile_metadata.tile_id.to_string(),
            include_expired: None,
        },
    )?;

    // Calculate fee distribution
    let developer_royalties = fees::developer_royalties() as u128;
    let developer_amount = required_fee.amount.multiply_ratio(developer_royalties, 100u128);
    let owner_amount = required_fee.amount - developer_amount;

    // Create bank messages for fee distribution
    let mut messages: Vec<BankMsg> = vec![];

    // Send developer fee
    messages.push(BankMsg::Send {
        to_address: fees::developer_address(),
        amount: vec![Coin {
            denom: required_fee.denom.clone(),
            amount: developer_amount,
        }],
    });

    // Send remaining fee to tile owner
    messages.push(BankMsg::Send {
        to_address: owner_response.owner,
        amount: vec![Coin {
            denom: required_fee.denom,
            amount: owner_amount,
        }],
    });

    // Return response with updated metadata and bank messages
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "set_pixel_color")
        .add_attribute("pixel_id", pixel_update.pixel_id.to_string())
        .add_attribute("color", format!("{:?}", pixel_update.color))
        .add_attribute("expiration", pixel_update.expiration.to_string())
        .add_attribute("duration", duration.to_string())
        .add_attribute("fee", required_fee.amount.to_string())
        .add_attribute("developer_fee", developer_amount)
        .add_attribute("owner_fee", owner_amount))
} 