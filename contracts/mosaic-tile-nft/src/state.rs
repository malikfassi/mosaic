use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CustomMsg, Empty};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use std::fmt;

// Constants
pub const PIXELS_PER_TILE: u32 = 100; // 10x10 pixels per tile
pub const TOTAL_TILES: u32 = 10000; // Total number of tiles
pub const TOTAL_PIXELS: u32 = PIXELS_PER_TILE * TOTAL_TILES; // 1M pixels
pub const PIXELS_PER_PACKED: u32 = 1; // Store one color per u32 (24 bits)
pub const CHUNKS_PER_TILE: u32 = PIXELS_PER_TILE / PIXELS_PER_PACKED;

#[cw_serde]
#[derive(Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Pack color into 24 bits (8 bits per component)
    pub fn pack(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    // Unpack color from 24 bits
    pub fn unpack(packed: u32) -> Self {
        Color {
            r: ((packed >> 16) & 0xFF) as u8,
            g: ((packed >> 8) & 0xFF) as u8,
            b: (packed & 0xFF) as u8,
        }
    }
}

#[cw_serde]
#[derive(Default)]
pub struct TileMetadata {
    // No color storage here, just a marker for CW721
}

impl CustomMsg for TileMetadata {}

#[cw_serde]
#[derive(Default)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

// CW721 base contract storage
pub type Cw721StorageType<'a> =
    cw721_base::Cw721Contract<'a, TileMetadata, TileMetadata, Empty, Empty>;
pub const TOKEN_COUNT: Item<u64> = Item::new("token_count");

// Mosaic-specific storage
// (tile_id, pixel_id) -> packed_color
pub const PIXEL_COLORS: Map<(u32, u32), u32> = Map::new("pixel_colors");
pub const DEVELOPER_FEE: Item<Coin> = Item::new("developer_fee");
pub const OWNER_FEE: Item<Coin> = Item::new("owner_fee");
pub const MINTER: Item<String> = Item::new("minter");
pub const DEVELOPER: Item<String> = Item::new("developer");
pub const POSITIONS: Map<u32, Position> = Map::new("positions");
pub const MOSAIC_CONFIG: Item<MosaicConfig> = Item::new("mosaic_config");

#[cw_serde]
pub struct MosaicConfig {
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub total_tiles: u32,
}

// Helper functions
pub fn validate_tile_id(tile_id: u32) -> bool {
    tile_id < TOTAL_TILES
}

pub fn validate_pixel_id(pixel_id: u32) -> bool {
    pixel_id < TOTAL_PIXELS
}

pub fn get_tile_id_from_pixel(pixel_id: u32) -> Option<u32> {
    if !validate_pixel_id(pixel_id) {
        return None;
    }
    Some(pixel_id / PIXELS_PER_TILE)
}

pub fn get_chunk_info(pixel_id: u32) -> Option<(u32, u32, u32)> {
    let tile_id = get_tile_id_from_pixel(pixel_id)?;
    let pixel_in_tile = pixel_id % PIXELS_PER_TILE;
    Some((tile_id, pixel_in_tile, 0))
}

pub fn pack_colors(colors: &[Color]) -> Vec<u32> {
    colors.iter().map(|color| color.pack()).collect()
}

pub fn unpack_colors(packed: u32) -> Vec<Color> {
    vec![Color::unpack(packed)]
}

pub fn get_pixel_position_in_tile(pixel_id: u32) -> Option<Position> {
    let pixel_in_tile = pixel_id % PIXELS_PER_TILE;
    let tile_width = (PIXELS_PER_TILE as f64).sqrt() as u32;
    Some(Position {
        x: pixel_in_tile % tile_width,
        y: pixel_in_tile / tile_width,
    })
}
