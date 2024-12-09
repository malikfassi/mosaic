use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub price_per_color_change: Uint128,
    pub nft_contract: String,
    pub color_change_cooldown: u64,
}

#[cw_serde]
pub struct ColorChange {
    pub x: u32,
    pub y: u32,
    pub color: String,
    pub last_change: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PIXEL_COLORS: Map<(u32, u32), ColorChange> = Map::new("pixel_colors"); 