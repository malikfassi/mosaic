use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use crate::state::{Config, ColorChange};

#[cw_serde]
pub struct InstantiateMsg {
    pub nft_contract: String,
    pub price_per_color_change: Uint128,
    pub color_change_cooldown: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetPixelColor {
        x: u32,
        y: u32,
        color: String,
    },
    UpdateConfig {
        price_per_color_change: Option<Uint128>,
        color_change_cooldown: Option<u64>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    GetConfig {},
    #[returns(ColorChange)]
    GetPixelColor { x: u32, y: u32 },
    #[returns(Vec<(u32, u32, ColorChange)>)]
    GetPixelColors { 
        start_after: Option<(u32, u32)>,
        limit: Option<u32>,
    },
} 