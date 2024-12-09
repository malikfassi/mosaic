use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128, Timestamp};
use mosaic_tile_nft::state::{Position, Color};
use crate::state::{ColorChangeEvent, TilePermissions, UserStatistics};

#[cw_serde]
pub struct InstantiateMsg {
    /// The mosaic NFT contract address
    pub nft_contract: String,
    /// The admin address that can update configuration
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
    /// Grant permissions in batch
    BatchGrantPermission {
        permissions: Vec<(Position, String, Option<Timestamp>)>,
    },
    /// Revoke permission to edit a tile's color
    RevokePermission {
        position: Position,
        editor: String,
    },
    /// Revoke permissions in batch
    BatchRevokePermission {
        permissions: Vec<(Position, String)>,
    },
    /// Set public editing status for a tile
    SetPublicEditing {
        position: Position,
        public_editing: bool,
        public_change_fee: Option<Uint128>,
    },
    /// Set public editing status for multiple tiles
    BatchSetPublicEditing {
        settings: Vec<(Position, bool, Option<Uint128>)>,
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
    /// Handle NFT transfer (called by NFT contract)
    HandleNftTransfer {
        token_id: String,
        from: String,
        to: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get contract configuration
    #[returns(ConfigResponse)]
    Config {},
    /// Get tile permissions
    #[returns(TilePermissionsResponse)]
    TilePermissions { position: Position },
    /// Get user statistics
    #[returns(UserStatisticsResponse)]
    UserStatistics { user: String },
    /// Get total fees collected
    #[returns(TotalFeesResponse)]
    TotalFees {},
    /// Check if a user can change a tile's color
    #[returns(CanChangeColorResponse)]
    CanChangeColor { position: Position, user: String },
    /// Get color change history for a tile
    #[returns(ColorHistoryResponse)]
    ColorHistory { position: Position, limit: Option<u32> },
    /// Get all permissions for a user
    #[returns(UserPermissionsResponse)]
    UserPermissions { user: String },
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
}

#[cw_serde]
pub struct TilePermissionsResponse {
    pub owner: Addr,
    pub allowed_editors: Vec<Addr>,
    pub public_editing: bool,
    pub permission_expiry: Option<Timestamp>,
    pub public_change_fee: Option<Uint128>,
}

#[cw_serde]
pub struct UserStatisticsResponse {
    pub total_color_changes: u64,
    pub total_fees_paid: Uint128,
    pub last_color_change: Option<Timestamp>,
    pub changes_in_window: u32,
    pub current_window_start: Option<Timestamp>,
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

#[cw_serde]
pub struct ColorHistoryResponse {
    pub history: Vec<ColorChangeEvent>,
}

#[cw_serde]
pub struct ColorChangeEvent {
    pub editor: Addr,
    pub from_color: Color,
    pub to_color: Color,
    pub timestamp: Timestamp,
    pub fee_paid: Option<Uint128>,
}

#[cw_serde]
pub struct UserPermissionsResponse {
    pub owned_tiles: Vec<Position>,
    pub editor_tiles: Vec<Position>,
    pub public_tiles: Vec<Position>,
} 