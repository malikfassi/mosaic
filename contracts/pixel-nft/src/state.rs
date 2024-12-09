use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub pixel_price: Uint128,
    pub color_change_price: Uint128,
    pub color_change_cooldown: u64,
    pub coloring_code_id: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const NFT_CONTRACT: Item<Addr> = Item::new("nft_contract");
pub const COLORING_CONTRACT: Item<Addr> = Item::new("coloring_contract");