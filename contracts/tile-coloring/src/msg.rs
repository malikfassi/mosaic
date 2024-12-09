use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Timestamp};
use mosaic_tile_nft::state::{Position, Color};
use crate::state::{ColorChangeEvent, TilePermissions, UserStatistics};

#[cw_serde]
pub struct InstantiateMsg {
    /// The NFT contract address
    pub nft_contract: String,
    /// The admin address
    pub admin: String,
    /// Cost per color change (0 for free changes)
    pub color_change_fee: Uint128,
    /// Maximum number of color changes per user per time window
    pub rate_limit: u32,
    /// Time window for rate limiting in seconds
    pub rate_limit_window: u64,
    /// Whether color changes require payment
    pub requires_payment: bool,
    /// Whether to enforce rate limiting
    pub rate_limiting_enabled: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Change a tile's color
    ChangeColor {
        position: Position,
        color: Color,
    },
    /// Grant permission to edit a tile's color
    GrantPermission {
        position: Position,
        editor: String,
        expires_at: Option<Timestamp>,
    },
    /// Revoke permission to edit a tile's color
    RevokePermission {
        position: Position,
        editor: String,
    },
    /// Enable/disable public editing for a tile
    SetPublicEditing {
        position: Position,
        public_editing: bool,
        public_change_fee: Option<Uint128>,
    },
    /// Update contract configuration
    UpdateConfig {
        nft_contract: Option<String>,
        admin: Option<String>,
        color_change_fee: Option<Uint128>,
        rate_limit: Option<u32>,
        rate_limit_window: Option<u64>,
        requires_payment: Option<bool>,
        rate_limiting_enabled: Option<bool>,
    },
    /// Withdraw collected fees
    WithdrawFees {
        amount: Option<Uint128>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(TilePermissionsResponse)]
    TilePermissions { position: Position },
    #[returns(ColorHistoryResponse)]
    ColorHistory { 
        position: Position,
        start_after: Option<Timestamp>,
        limit: Option<u32>,
    },
    #[returns(UserStatisticsResponse)]
    UserStatistics { address: String },
    #[returns(TotalFeesResponse)]
    TotalFees {},
    #[returns(CanChangeColorResponse)]
    CanChangeColor { 
        position: Position,
        editor: String,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub nft_contract: Addr,
    pub admin: Addr,
    pub color_change_fee: Uint128,
    pub rate_limit: u32,
    pub rate_limit_window: u64,
    pub requires_payment: bool,
    pub rate_limiting_enabled: bool,
    pub total_tiles_modified: u64,
}

#[cw_serde]
pub struct TilePermissionsResponse {
    pub position: Position,
    pub permissions: TilePermissions,
}

#[cw_serde]
pub struct ColorHistoryResponse {
    pub position: Position,
    pub history: Vec<ColorChangeEvent>,
}

#[cw_serde]
pub struct UserStatisticsResponse {
    pub address: Addr,
    pub statistics: UserStatistics,
}

#[cw_serde]
pub struct TotalFeesResponse {
    pub total_fees: Uint128,
}

#[cw_serde]
pub struct CanChangeColorResponse {
    pub can_change: bool,
    pub reason: Option<String>,
    pub required_fee: Option<Uint128>,
} 