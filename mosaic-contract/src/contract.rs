use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use sg_std::StargazeMsgWrapper;
use crate::{
    error::ContractError,
    msg::PixelUpdate,
    types::TileMetadata,
    constants::fees,
};

pub fn execute_set_pixel_color(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    current_tile_metadata: Vec<u8>,
    pixel_update: PixelUpdate,
) -> Result<Response<StargazeMsgWrapper>, ContractError> {
    // Validate payment
    if info.funds.len() != 1 || info.funds[0] != fees::base_fee() {
        return Err(ContractError::InvalidFee {
            expected: fees::base_fee(),
            received: info.funds.get(0).cloned(),
        });
    }

    // Parse the current tile metadata
    let mut tile_metadata = TileMetadata::from_bytes(&current_tile_metadata)
        .map_err(|e| ContractError::InvalidPixelUpdate(e.to_string()))?;

    // Verify pixel is in tile range
    if !crate::types::is_pixel_in_tile(pixel_update.pixel_id, tile_metadata.tile_id) {
        return Err(ContractError::PixelOutOfRange {});
    }

    // Get current pixel metadata
    let current_pixel = tile_metadata.get_pixel(pixel_update.pixel_id)
        .map_err(|e| ContractError::InvalidPixelUpdate(e))?;

    // Verify pixel is not expired
    if current_pixel.expiration > 0 {
        return Err(ContractError::InvalidPixelUpdate("Pixel is currently locked".to_string()));
    }

    // Update the pixel
    tile_metadata.update_pixel(
        pixel_update.pixel_id,
        pixel_update.color,
        pixel_update.expiration,
    ).map_err(|e| ContractError::InvalidPixelUpdate(e))?;

    // Return response with updated metadata
    Ok(Response::new()
        .set_data(tile_metadata.to_bytes())
        .add_attribute("action", "set_pixel_color")
        .add_attribute("pixel_id", pixel_update.pixel_id.to_string())
        .add_attribute("color", format!("{:?}", pixel_update.color))
        .add_attribute("expiration", pixel_update.expiration.to_string()))
} 