use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub width: u32,
    pub height: u32,
    pub price_per_pixel: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    BuyPixel { x: u32, y: u32 },
    SetPixelColor { x: u32, y: u32, color: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetPixel { x: u32, y: u32 },
    GetCanvas {},
    GetConfig {},
    GetOwnerPixels { owner: String },
} 