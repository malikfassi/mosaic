use cosmwasm_std::{Addr, Uint128};
use mosaic_tile_nft::state::{Position, Color};

// Contract addresses
pub const MOCK_NFT_CONTRACT: &str = "contract0";
pub const MOCK_ADMIN: &str = "admin0";
pub const MOCK_OWNER: &str = "owner0";
pub const MOCK_USER1: &str = "user1";
pub const MOCK_USER2: &str = "user2";

// Token configuration
pub const DEFAULT_COLOR_CHANGE_FEE: u128 = 1_000_000;
pub const DEFAULT_RATE_LIMIT: u32 = 10;
pub const DEFAULT_RATE_LIMIT_WINDOW: u64 = 3600; // 1 hour

// Test positions
pub const TEST_POSITIONS: &[(u32, u32)] = &[
    (0, 0),
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 4),
];

// Test colors
pub const TEST_COLORS: &[(u8, u8, u8)] = &[
    (255, 0, 0),   // Red
    (0, 255, 0),   // Green
    (0, 0, 255),   // Blue
    (255, 255, 0), // Yellow
    (255, 0, 255), // Magenta
];

// Helper functions
pub fn mock_nft_contract() -> Addr {
    Addr::unchecked(MOCK_NFT_CONTRACT)
}

pub fn mock_admin() -> Addr {
    Addr::unchecked(MOCK_ADMIN)
}

pub fn mock_owner() -> Addr {
    Addr::unchecked(MOCK_OWNER)
}

pub fn mock_user1() -> Addr {
    Addr::unchecked(MOCK_USER1)
}

pub fn mock_user2() -> Addr {
    Addr::unchecked(MOCK_USER2)
}

pub fn default_color_change_fee() -> Uint128 {
    Uint128::from(DEFAULT_COLOR_CHANGE_FEE)
}

pub fn test_position(index: usize) -> Position {
    let (x, y) = TEST_POSITIONS[index % TEST_POSITIONS.len()];
    Position { x, y }
}

pub fn test_color(index: usize) -> Color {
    let (r, g, b) = TEST_COLORS[index % TEST_COLORS.len()];
    Color { r, g, b }
}

pub fn test_token_id(position: &Position) -> String {
    format!("tile_{}_{}",  position.x, position.y)
} 