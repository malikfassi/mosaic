use crate::state::{Color, Position, TileMetadata};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Empty};
use cw721_base::msg::{ExecuteMsg as Cw721ExecuteMsg, QueryMsg as Cw721QueryMsg};
use cw_utils::Expiration;
use sg721::CollectionInfo;

#[cw_serde]
pub struct InstantiateMsg {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,
    /// The minter address
    pub minter: String,
    /// The developer address (receives fees)
    pub developer: String,
    /// Collection info
    pub collection_info: CollectionInfo<Empty>,
    /// Fee for setting pixel color (goes to developer)
    pub developer_fee: Coin,
    /// Fee for setting pixel color (goes to tile owner)
    pub owner_fee: Coin,
}

#[cw_serde]
pub struct PixelUpdate {
    pub pixel_id: u32,
    pub color: Color,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// CW721 execute messages
    Cw721(Box<Cw721ExecuteMsg<TileMetadata, Empty>>),
    
    /// Mint a new tile NFT
    MintTile {
        tile_id: u32,
        owner: String,
    },
    
    /// Set a pixel's color
    SetPixelColor {
        pixel_id: u32,
        color: Color,
    },
    
    /// Batch update multiple pixels
    BatchSetPixels {
        updates: Vec<PixelUpdate>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// CW721 query messages
    #[returns(cw721_base::msg::QueryMsg<TileMetadata>)]
    Cw721(Box<Cw721QueryMsg<TileMetadata>>),
    
    /// Query tile state by ID
    #[returns(TileStateResponse)]
    TileState { tile_id: u32 },
    
    /// Query multiple tiles at once
    #[returns(TilesStateResponse)]
    TilesState { tile_ids: Vec<u32> },
    
    /// Query mosaic state
    #[returns(MosaicStateResponse)]
    MosaicState {},

    /// Query pixel state
    #[returns(PixelStateResponse)]
    PixelState { pixel_id: u32 },

    /// Query multiple pixels with pagination
    #[returns(Vec<PixelStateResponse>)]
    PixelsState { 
        pixel_ids: Vec<u32>,
        start_after: Option<u32>,
        limit: Option<u32>,
    },

    /// Query all pixels in a tile
    #[returns(TilePixelsResponse)]
    TilePixels { tile_id: u32 },

    /// Query all pixels in multiple tiles
    #[returns(Vec<TilePixelsResponse>)]
    BatchTilePixels { tile_ids: Vec<u32> },
}

#[cw_serde]
pub struct TileStateResponse {
    pub owner: String,
    pub tile_id: u32,
    pub pixel_colors: Vec<Color>,
}

#[cw_serde]
pub struct TilesStateResponse {
    pub tiles: Vec<TileStateResponse>,
}

#[cw_serde]
pub struct PixelStateResponse {
    pub tile_id: u32,
    pub owner: String,
    pub color: Color,
    pub position: Position,
}

#[cw_serde]
pub struct MosaicStateResponse {
    pub total_tiles_minted: u64,
    pub developer_fee: Coin,
    pub owner_fee: Coin,
}

#[cw_serde]
pub struct TilePixelsResponse {
    pub tile_id: u32,
    pub owner: String,
    pub pixels: Vec<PixelStateResponse>,
}
