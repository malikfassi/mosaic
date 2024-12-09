use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub nft_contract: Addr,
    pub price_per_color_change: Uint128,
    pub color_change_cooldown: u64,  // Time in seconds between color changes
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ColorChange {
    pub last_change: u64,  // Timestamp of last color change
    pub color: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const COLOR_CHANGES: Map<(u32, u32), ColorChange> = Map::new("color_changes"); 