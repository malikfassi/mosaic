use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use mosaic_tile_nft::state::{Position, Color};

#[cw_serde]
pub struct InstantiateMsg {
    /// The mosaic NFT contract address
    pub mosaic_nft_address: String,
    /// The payment address where funds are sent
    pub payment_address: String,
    /// The cost of minting one tile
    pub unit_price: Uint128,
    /// The maximum number of tiles that can be minted in one transaction
    pub max_batch_size: u32,
    /// Whether random minting is enabled
    pub random_minting_enabled: bool,
    /// Whether position-based minting is enabled
    pub position_minting_enabled: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Mint a tile at a random position
    MintRandom {
        /// Initial color for the tile
        color: Color,
    },
    /// Mint a tile at a specific position
    MintPosition {
        /// Position to mint the tile at
        position: Position,
        /// Initial color for the tile
        color: Color,
    },
    /// Mint multiple tiles at random positions
    BatchMintRandom {
        /// Number of tiles to mint
        count: u32,
        /// Initial colors for the tiles
        colors: Vec<Color>,
    },
    /// Mint multiple tiles at specific positions
    BatchMintPositions {
        /// Positions and colors for each tile
        mints: Vec<(Position, Color)>,
    },
    /// Update contract configuration
    UpdateConfig {
        /// Optional new mosaic NFT contract address
        mosaic_nft_address: Option<String>,
        /// Optional new payment address
        payment_address: Option<String>,
        /// Optional new unit price
        unit_price: Option<Uint128>,
        /// Optional new max batch size
        max_batch_size: Option<u32>,
        /// Optional flag to enable/disable random minting
        random_minting_enabled: Option<bool>,
        /// Optional flag to enable/disable position minting
        position_minting_enabled: Option<bool>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(MintPositionResponse)]
    MintPosition { position: Position },
    #[returns(MintCountResponse)]
    MintCount {},
    #[returns(MintPriceResponse)]
    MintPrice { count: u32 },
    #[returns(MintablePositionsResponse)]
    MintablePositions { start_after: Option<Position>, limit: Option<u32> },
}

#[cw_serde]
pub struct ConfigResponse {
    pub mosaic_nft_address: Addr,
    pub payment_address: Addr,
    pub unit_price: Uint128,
    pub max_batch_size: u32,
    pub random_minting_enabled: bool,
    pub position_minting_enabled: bool,
}

#[cw_serde]
pub struct MintPositionResponse {
    pub position: Position,
    pub is_minted: bool,
    pub token_id: Option<String>,
}

#[cw_serde]
pub struct MintCountResponse {
    pub total_minted: u32,
}

#[cw_serde]
pub struct MintPriceResponse {
    pub price: Uint128,
}

#[cw_serde]
pub struct MintablePositionsResponse {
    pub positions: Vec<Position>,
} 