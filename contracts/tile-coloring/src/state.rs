use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Timestamp};
use cw_storage_plus::{Item, Map};
use mosaic_tile_nft::state::{Position, Color};

#[cw_serde]
pub struct Config {
    /// The mosaic NFT contract address
    pub nft_contract: Addr,
    /// The admin address that can update configuration
    pub admin: Addr,
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
pub struct UserStatistics {
    /// Total number of color changes made by the user
    pub total_color_changes: u64,
    /// Total fees paid by the user
    pub total_fees_paid: Uint128,
    /// Last color change timestamp
    pub last_color_change: Option<Timestamp>,
    /// Number of color changes in current window
    pub changes_in_window: u32,
    /// Start of current rate limit window
    pub current_window_start: Option<Timestamp>,
}

#[cw_serde]
pub struct TilePermissions {
    /// The owner of the tile (from NFT contract)
    pub owner: Addr,
    /// Addresses allowed to change the tile's color
    pub allowed_editors: Vec<Addr>,
    /// Whether anyone can change the color (public)
    pub public_editing: bool,
    /// Optional expiry time for permissions
    pub permission_expiry: Option<Timestamp>,
    /// Optional fee for public color changes
    pub public_change_fee: Option<Uint128>,
}

#[cw_serde]
pub struct ColorChangeEvent {
    /// Who changed the color
    pub editor: Addr,
    /// Previous color
    pub from_color: Color,
    /// New color
    pub to_color: Color,
    /// When the change occurred
    pub timestamp: Timestamp,
    /// Fee paid (if any)
    pub fee_paid: Option<Uint128>,
}

// Store contract configuration
pub const CONFIG: Item<Config> = Item::new("config");

// Store user statistics
// Key: user address
pub const USER_STATS: Map<&Addr, UserStatistics> = Map::new("user_stats");

// Store tile permissions
// Key: (x, y) coordinates
pub const TILE_PERMISSIONS: Map<(u32, u32), TilePermissions> = Map::new("tile_permissions");

// Store color change history
// Key: (x, y) coordinates
pub const COLOR_HISTORY: Map<(u32, u32), Vec<ColorChangeEvent>> = Map::new("color_history");

// Store total fees collected
pub const TOTAL_FEES: Item<Uint128> = Item::new("total_fees");

// Helper function to check if a user can change a tile's color
pub fn can_change_color(
    permissions: &TilePermissions,
    user: &Addr,
    current_time: &Timestamp,
) -> bool {
    // Owner can always change color
    if permissions.owner == *user {
        return true;
    }

    // Check if permissions have expired
    if let Some(expiry) = permissions.permission_expiry {
        if expiry < *current_time {
            return false;
        }
    }

    // Check if user is in allowed editors
    if permissions.allowed_editors.contains(user) {
        return true;
    }

    // Check if public editing is enabled
    permissions.public_editing
}

// Helper function to check rate limit for a user
pub fn check_rate_limit(
    stats: &UserStatistics,
    config: &Config,
    current_time: Timestamp,
) -> bool {
    if !config.rate_limiting_enabled {
        return true;
    }

    let window_start = stats.current_window_start.unwrap_or(current_time);
    let window_end = window_start.plus_seconds(config.rate_limit_window);

    if current_time > window_end {
        // Window has expired, user can make changes
        true
    } else {
        // Check if user has exceeded rate limit in current window
        stats.changes_in_window < config.rate_limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_change_permission() {
        let owner = Addr::unchecked("owner");
        let editor = Addr::unchecked("editor");
        let random = Addr::unchecked("random");
        let current_time = Timestamp::from_seconds(1000);

        let permissions = TilePermissions {
            owner: owner.clone(),
            allowed_editors: vec![editor.clone()],
            public_editing: false,
            permission_expiry: None,
            public_change_fee: None,
        };

        // Owner can always change
        assert!(can_change_color(&permissions, &owner, &current_time));
        // Allowed editor can change
        assert!(can_change_color(&permissions, &editor, &current_time));
        // Random user cannot change
        assert!(!can_change_color(&permissions, &random, &current_time));

        // Test with expired permissions
        let expired_permissions = TilePermissions {
            owner: owner.clone(),
            allowed_editors: vec![editor.clone()],
            public_editing: false,
            permission_expiry: Some(Timestamp::from_seconds(900)),
            public_change_fee: None,
        };

        // Owner can still change
        assert!(can_change_color(&expired_permissions, &owner, &current_time));
        // Editor cannot change after expiry
        assert!(!can_change_color(&expired_permissions, &editor, &current_time));
    }

    #[test]
    fn test_rate_limiting() {
        let config = Config {
            nft_contract: Addr::unchecked("nft"),
            admin: Addr::unchecked("admin"),
            color_change_fee: Uint128::zero(),
            rate_limit: 5,
            rate_limit_window: 3600, // 1 hour
            requires_payment: false,
            rate_limiting_enabled: true,
        };

        let current_time = Timestamp::from_seconds(1000);

        // Test new user
        let new_user = UserStatistics {
            total_color_changes: 0,
            total_fees_paid: Uint128::zero(),
            last_color_change: None,
            changes_in_window: 0,
            current_window_start: None,
        };
        assert!(check_rate_limit(&new_user, &config, current_time));

        // Test user at limit
        let user_at_limit = UserStatistics {
            total_color_changes: 5,
            total_fees_paid: Uint128::zero(),
            last_color_change: Some(current_time),
            changes_in_window: 5,
            current_window_start: Some(current_time),
        };
        assert!(!check_rate_limit(&user_at_limit, &config, current_time));

        // Test user after window expiry
        let next_window = current_time.plus_seconds(3601);
        assert!(check_rate_limit(&user_at_limit, &config, next_window));
    }
} 