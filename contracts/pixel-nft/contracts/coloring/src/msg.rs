use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Empty, Uint128};
use sg721_base::msg::ExecuteMsg as Sg721ExecuteMsg;
use sg_metadata::Metadata;
use crate::state::{ColorChange};

#[cw_serde]
pub struct InstantiateMsg {
    pub price_per_color_change: Uint128,
    pub nft_contract: String,
    pub color_change_cooldown: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Base(Sg721ExecuteMsg<Metadata, Empty>),
    UpdateConfig {
        price_per_color_change: Option<Uint128>,
        nft_contract: Option<String>,
        color_change_cooldown: Option<u64>,
    },
    ChangeColor {
        token_id: String,
        x: u32,
        y: u32,
        color: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::Config)]
    GetConfig {},
    #[returns(Option<ColorChange>)]
    GetPixelColor { x: u32, y: u32 },
    #[returns(Vec<ColorChange>)]
    ListPixelColors {
        start_after: Option<(u32, u32)>,
        limit: Option<u32>,
    },
} 