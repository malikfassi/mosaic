use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub pixel_price: Uint128,
    pub color_change_price: Uint128,
    pub color_change_cooldown: u64,
    pub nft_code_id: u64,
    pub coloring_code_id: u64,
    pub collection_image: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        pixel_price: Option<u128>,
        color_change_price: Option<u128>,
        color_change_cooldown: Option<u64>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    GetConfig {},
    #[returns((Option<Addr>, Option<Addr>))]
    GetContracts {},
} 