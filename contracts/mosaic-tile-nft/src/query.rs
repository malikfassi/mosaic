use cosmwasm_std::{Deps, StdResult};

use crate::{
    msg::{TileStateResponse, TilesStateResponse, PixelStateResponse, MosaicStateResponse, TilePixelsResponse},
    state::{
        Cw721StorageType, DEVELOPER_FEE, OWNER_FEE, PIXEL_COLORS, TOKEN_COUNT,
        validate_tile_id, validate_pixel_id, get_tile_id_from_pixel, get_pixel_position_in_tile,
        Color, PIXELS_PER_TILE,
    },
};

pub fn query_tile_state(deps: Deps, tile_id: u32) -> StdResult<TileStateResponse> {
    if !validate_tile_id(tile_id) {
        return Err(cosmwasm_std::StdError::generic_err("Invalid tile ID"));
    }

    let contract = Cw721StorageType::default();
    let token = contract.tokens.load(deps.storage, &tile_id.to_string())?;

    let mut pixel_colors = Vec::with_capacity(PIXELS_PER_TILE as usize);
    for pixel_in_tile in 0..PIXELS_PER_TILE {
        let packed_color = PIXEL_COLORS
            .may_load(deps.storage, (tile_id, pixel_in_tile))?
            .unwrap_or(0);
        pixel_colors.push(Color::unpack(packed_color));
    }

    Ok(TileStateResponse {
        owner: token.owner.to_string(),
        tile_id,
        pixel_colors,
    })
}

pub fn query_tiles_state(deps: Deps, tile_ids: Vec<u32>) -> StdResult<TilesStateResponse> {
    let mut tiles = Vec::with_capacity(tile_ids.len());
    for tile_id in tile_ids {
        tiles.push(query_tile_state(deps, tile_id)?);
    }
    Ok(TilesStateResponse { tiles })
}

pub fn query_pixel_state(deps: Deps, pixel_id: u32) -> StdResult<PixelStateResponse> {
    if !validate_pixel_id(pixel_id) {
        return Err(cosmwasm_std::StdError::generic_err("Invalid pixel ID"));
    }

    let tile_id = get_tile_id_from_pixel(pixel_id)
        .ok_or_else(|| cosmwasm_std::StdError::generic_err("Invalid pixel ID"))?;
    let pixel_in_tile = pixel_id % PIXELS_PER_TILE;

    let contract = Cw721StorageType::default();
    let token = contract.tokens.load(deps.storage, &tile_id.to_string())?;

    let packed_color = PIXEL_COLORS
        .may_load(deps.storage, (tile_id, pixel_in_tile))?
        .unwrap_or(0);

    Ok(PixelStateResponse {
        tile_id,
        owner: token.owner.to_string(),
        color: Color::unpack(packed_color),
        position: get_pixel_position_in_tile(pixel_id)
            .ok_or_else(|| cosmwasm_std::StdError::generic_err("Invalid pixel position"))?,
    })
}

pub fn query_pixels_state(
    deps: Deps,
    pixel_ids: Vec<u32>,
    start_after: Option<u32>,
    limit: Option<u32>,
) -> StdResult<Vec<PixelStateResponse>> {
    let start = start_after.map(|s| s + 1).unwrap_or(0);
    let limit = limit.unwrap_or(pixel_ids.len() as u32) as usize;

    let pixels: Vec<PixelStateResponse> = pixel_ids
        .into_iter()
        .skip(start as usize)
        .take(limit)
        .map(|pixel_id| query_pixel_state(deps, pixel_id))
        .collect::<StdResult<_>>()?;

    Ok(pixels)
}

pub fn query_mosaic_state(deps: Deps) -> StdResult<MosaicStateResponse> {
    let total_tiles_minted = TOKEN_COUNT.load(deps.storage)?;
    let developer_fee = DEVELOPER_FEE.load(deps.storage)?;
    let owner_fee = OWNER_FEE.load(deps.storage)?;

    Ok(MosaicStateResponse {
        total_tiles_minted,
        developer_fee,
        owner_fee,
    })
}

pub fn query_tile_pixels(deps: Deps, tile_id: u32) -> StdResult<TilePixelsResponse> {
    if !validate_tile_id(tile_id) {
        return Err(cosmwasm_std::StdError::generic_err("Invalid tile ID"));
    }

    let contract = Cw721StorageType::default();
    let token = contract.tokens.load(deps.storage, &tile_id.to_string())?;

    let mut pixels = Vec::with_capacity(PIXELS_PER_TILE as usize);
    for pixel_in_tile in 0..PIXELS_PER_TILE {
        let pixel_id = tile_id * PIXELS_PER_TILE + pixel_in_tile;
        let packed_color = PIXEL_COLORS
            .may_load(deps.storage, (tile_id, pixel_in_tile))?
            .unwrap_or(0);

        pixels.push(PixelStateResponse {
            tile_id,
            owner: token.owner.to_string(),
            color: Color::unpack(packed_color),
            position: get_pixel_position_in_tile(pixel_id)
                .ok_or_else(|| cosmwasm_std::StdError::generic_err("Invalid pixel position"))?,
        });
    }

    Ok(TilePixelsResponse {
        tile_id,
        owner: token.owner.to_string(),
        pixels,
    })
}

pub fn query_batch_tile_pixels(deps: Deps, tile_ids: Vec<u32>) -> StdResult<Vec<TilePixelsResponse>> {
    let mut responses = Vec::with_capacity(tile_ids.len());
    for tile_id in tile_ids {
        responses.push(query_tile_pixels(deps, tile_id)?);
    }
    Ok(responses)
} 