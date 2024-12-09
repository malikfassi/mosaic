use crate::PixelExtension;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Empty;
use cw721::Expiration;
use sg721::CollectionInfo;

#[cw_serde]
pub struct InstantiateMsg {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,
    /// The minter address
    pub minter: String,
    /// Collection info
    pub collection_info: CollectionInfo<Empty>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PixelResponse)]
    Pixel { x: u32, y: u32 },
    #[returns(PixelsResponse)]
    Pixels { start_after: Option<(u32, u32)>, limit: Option<u32> },
}

#[cw_serde]
pub struct PixelResponse {
    pub token_id: String,
    pub owner: String,
    pub color: String,
}

#[cw_serde]
pub struct PixelsResponse {
    pub pixels: Vec<(u32, u32, PixelResponse)>,
} 