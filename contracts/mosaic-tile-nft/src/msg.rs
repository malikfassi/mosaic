use crate::state::{Color, Position, TileMetadata};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;
use cw721::{Expiration, OperatorsResponse};

#[cw_serde]
pub enum ExecuteMsg {
    /// Mint a new tile NFT
    MintTile {
        token_id: String,
        owner: String,
        position: Position,
        color: Color,
    },
    /// Update the color of an existing tile
    UpdateTileColor {
        token_id: String,
        color: Color,
    },
    /// Freeze all token metadata
    FreezeTokenMetadata {},
    /// Enable updatable metadata
    EnableUpdatable {},
    /// CW721 messages
    TransferNft {
        recipient: String,
        token_id: String,
    },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: String,
        token_id: String,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get information about a specific tile
    #[returns(TileInfoResponse)]
    TileInfo { token_id: String },
    /// Get tile at a specific position
    #[returns(TileAtPositionResponse)]
    TileAtPosition { position: Position },
    /// Check if metadata is updatable
    #[returns(EnableUpdatableResponse)]
    EnableUpdatable {},
    /// Check if metadata is frozen
    #[returns(bool)]
    FreezeTokenMetadata {},
    /// CW721 queries
    #[returns(cw721::OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    #[returns(cw721::ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    #[returns(cw721::ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    #[returns(cw721::OperatorResponse)]
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    #[returns(OperatorsResponse)]
    AllOperators {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(cw721::NumTokensResponse)]
    NumTokens {},
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},
    #[returns(cw721::NftInfoResponse<TileMetadata>)]
    NftInfo { token_id: String },
    #[returns(cw721::AllNftInfoResponse<TileMetadata>)]
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    #[returns(cw721::TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(cw721::TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct TileInfoResponse {
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
