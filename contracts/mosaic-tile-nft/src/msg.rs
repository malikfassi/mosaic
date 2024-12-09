use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::state::{Position, Color, TileMetadata};

#[cw_serde]
pub enum ExecuteMsg {
    // Tile-specific messages
    MintTile {
        token_id: String,
        owner: String,
        position: Position,
        color: Color,
    },
    UpdateTileColor {
        token_id: String,
        color: Color,
    },
    FreezeTokenMetadata {},
    EnableUpdatable {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TileInfoResponse)]
    TileInfo { token_id: String },
    #[returns(TileAtPositionResponse)]
    TileAtPosition { position: Position },
    #[returns(EnableUpdatableResponse)]
    EnableUpdatable {},
    #[returns(bool)]
    FreezeTokenMetadata {},
}

#[cw_serde]
pub struct TileInfoResponse {
    pub token_id: String,
    pub owner: String,
    pub metadata: TileMetadata,
}

#[cw_serde]
pub struct TileAtPositionResponse {
    pub token_id: Option<String>,
}

#[cw_serde]
pub struct EnableUpdatableResponse {
    pub enabled: bool,
}
