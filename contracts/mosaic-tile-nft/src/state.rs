use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map, Index, IndexList, IndexedMap, MultiIndex};

// Constants
pub const MAX_POSITION: u32 = 99; // 100x100 grid (0-99)

// Base state from sg721-updatable
pub const FROZEN_TOKEN_METADATA: Item<bool> = Item::new("frozen_token_metadata");
pub const ENABLE_UPDATABLE: Item<bool> = Item::new("enable_updatable");

// Tile-specific structures
#[cw_serde]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[cw_serde]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[cw_serde]
pub struct TileMetadata {
    pub position: Position,
    pub current_color: Color,
}

// Indexes for efficient querying
pub struct TileIndexes<'a> {
    // Index tiles by position for efficient position-based lookups
    pub position: MultiIndex<'a, (u32, u32), TileMetadata, String>,
}

impl<'a> IndexList<TileMetadata> for TileIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TileMetadata>> + '_> {
        let v: Vec<&dyn Index<TileMetadata>> = vec![&self.position];
        Box::new(v.into_iter())
    }
}

// Create indexes
pub fn tile_indexes<'a>() -> TileIndexes<'a> {
    TileIndexes {
        position: MultiIndex::new(
            |_pk: &[u8], d: &TileMetadata| (d.position.x, d.position.y),
            "tile_metadata",
            "tile_metadata__position",
        ),
    }
}

// Optimized storage - using IndexedMap for efficient queries
pub const TILE_METADATA: IndexedMap<&str, TileMetadata, TileIndexes> = 
    IndexedMap::new(
        "tile_metadata",
        tile_indexes,
    );

// Keep position to token mapping for direct lookups
pub const POSITION_TO_TOKEN: Map<(u32, u32), String> = Map::new("position_to_token");
