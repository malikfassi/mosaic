use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub canvas_size: u32,
    pub pixel_price: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pixel {
    pub owner: Addr,
    pub color: String,
    pub last_updated: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PIXELS: Map<(u32, u32), Pixel> = Map::new("pixels");

// Key: owner address, Value: list of pixel coordinates owned
pub const OWNER_PIXELS: Map<Addr, Vec<(u32, u32)>> = Map::new("owner_pixels"); 