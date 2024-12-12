use crate::{
    error::ContractError,
    state::{Position, PIXELS_PER_TILE, POSITIONS, TOTAL_TILES},
};
use cosmwasm_std::DepsMut;

pub fn validate_tile_id(token_num: u32) -> bool {
    token_num < TOTAL_TILES
}

pub fn validate_pixel_id(pixel_id: u32) -> bool {
    pixel_id < TOTAL_TILES * PIXELS_PER_TILE
}

pub fn get_tile_id_from_pixel(pixel_id: u32) -> Option<u32> {
    if !validate_pixel_id(pixel_id) {
        return None;
    }
    Some(pixel_id / PIXELS_PER_TILE)
}

pub fn validate_position(deps: &DepsMut, position: &Position) -> Result<(), ContractError> {
    // Check if position is within bounds
    if position.x >= TOTAL_TILES || position.y >= 1 {
        return Err(ContractError::InvalidPosition {
            x: position.x,
            y: position.y,
        });
    }

    // Check if position is already taken
    let position_key = position.x;
    if POSITIONS.has(deps.storage, position_key) {
        return Err(ContractError::PositionTaken {
            x: position.x,
            y: position.y,
        });
    }

    Ok(())
}
