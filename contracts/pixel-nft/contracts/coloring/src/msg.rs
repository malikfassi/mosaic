use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Uint128;
use crate::state::{Config, ColorChange};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub nft_contract: String,
    pub price_per_color_change: Uint128,
    pub color_change_cooldown: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetPixelColor { x: u32, y: u32, color: String },
    UpdateConfig {
        price_per_color_change: Option<Uint128>,
        color_change_cooldown: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetPixelColor { x: u32, y: u32 },
    GetPixelColors {
        start_after: Option<(u32, u32)>,
        limit: Option<u32>,
    },
} 