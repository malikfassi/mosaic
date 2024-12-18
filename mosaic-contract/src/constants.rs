use cosmwasm_std::{Coin, Uint128};
use lazy_static::lazy_static;

// Read config.json at compile time
const CONFIG_STR: &str = include_str!("../../config.json");

lazy_static! {
    static ref CONFIG: Config = serde_json::from_str(CONFIG_STR).expect("Failed to parse config.json");
}

pub mod fees {
    use super::*;

    pub fn base_fee() -> Coin {
        Coin {
            amount: Uint128::from_str(&CONFIG.mosaic.fees.base_fee.amount)
                .expect("Invalid base fee amount"),
            denom: CONFIG.mosaic.fees.base_fee.denom.clone(),
        }
    }

    pub fn mint_price() -> Coin {
        Coin {
            amount: Uint128::from_str(&CONFIG.mosaic.fees.mint_price.amount)
                .expect("Invalid mint price amount"),
            denom: CONFIG.mosaic.fees.mint_price.denom.clone(),
        }
    }

    pub fn developer_royalties() -> u8 {
        CONFIG.mosaic.fees.developer_royalties
    }
}

pub mod tile {
    use super::*;

    pub fn total_tiles() -> u32 {
        CONFIG.mosaic.tile.total_tiles
    }

    pub fn total_pixels_per_tile() -> u32 {
        CONFIG.mosaic.tile.total_pixels_per_tile
    }
}

// Config structs
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
struct Config {
    mosaic: MosaicConfig,
}

#[derive(Deserialize)]
struct MosaicConfig {
    fees: Fees,
    tile: Tile,
}

#[derive(Deserialize)]
struct Fees {
    base_fee: Fee,
    developer_royalties: u8,
    mint_price: Fee,
}

#[derive(Deserialize)]
struct Fee {
    amount: String,
    denom: String,
}

#[derive(Deserialize)]
struct Tile {
    total_tiles: u32,
    total_pixels_per_tile: u32,
} 