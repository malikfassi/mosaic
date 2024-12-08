use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub width: u32,
    pub height: u32,
    pub price_per_pixel: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pixel {
    pub owner: Addr,
    pub color: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PIXELS: Map<(u32, u32), Pixel> = Map::new("pixels");
pub const OWNER_PIXELS: Map<Addr, Vec<(u32, u32)>> = Map::new("owner_pixels"); 