use cosmwasm_std::{Coin, Uint128};
use lazy_static::lazy_static;

// Read config.json at compile time
const CONFIG_STR: &str = include_str!("../../config.json");

lazy_static! {
    static ref CONFIG: Config = serde_json::from_str(CONFIG_STR).expect("Failed to parse config.json");
}

pub mod fees {
    use super::*;
    use std::cmp::max;

    // Time constants in seconds
    pub const HOUR: u64 = 3600;
    pub const DAY: u64 = 24 * HOUR;

    // Fee tiers in ustars (1 STARS = 1_000_000 ustars)
    pub const FEE_TIER_1H: u128 = 5_000_000;  // 5 STARS for < 1h
    pub const FEE_TIER_12H: u128 = 10_000_000; // 10 STARS for < 12h
    pub const FEE_TIER_24H: u128 = 15_000_000; // 15 STARS for < 24h

    pub fn calculate_fee(expiration_duration: u64) -> Coin {
        let amount = if expiration_duration < HOUR {
            FEE_TIER_1H
        } else if expiration_duration < 12 * HOUR {
            FEE_TIER_12H
        } else if expiration_duration < DAY {
            FEE_TIER_24H
        } else {
            // For durations > 24h, scale quadratically with per-second granularity
            // Base fee is 15 STARS for 24h
            // Formula: base_fee * (duration/24h)^2
            let seconds_ratio = expiration_duration as f64 / DAY as f64;
            let scale = seconds_ratio * seconds_ratio; // quadratic scaling
            let scaled_amount = (FEE_TIER_24H as f64 * scale) as u128;
            max(scaled_amount, FEE_TIER_24H) // Never go below base fee
        };

        Coin {
            amount: Uint128::from(amount),
            denom: CONFIG.mosaic.fees.base_fee.denom.clone(),
        }
    }

    pub fn developer_royalties() -> u8 {
        CONFIG.mosaic.fees.developer_royalties
    }

    pub fn developer_address() -> String {
        CONFIG.mosaic.fees.developer_address.clone()
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
    developer_address: String,
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