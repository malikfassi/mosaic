use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use mosaic_tile_nft::state::{Position, MAX_POSITION};

#[cw_serde]
pub struct Config {
    /// The mosaic NFT contract address
    pub mosaic_nft_address: Addr,
    /// The payment address where funds are sent
    pub payment_address: Addr,
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
pub struct MintPosition {
    pub position: Position,
    pub is_minted: bool,
}

// Store configuration
pub const CONFIG: Item<Config> = Item::new("config");

// Track which positions have been minted
// Key is (x, y) coordinates, value is token_id if minted
pub const POSITION_TOKENS: Map<(u32, u32), Option<String>> = Map::new("position_tokens");

// Track total number of minted tiles
pub const TOTAL_MINTED: Item<u32> = Item::new("total_minted");

// Track the next available position for random minting
pub const NEXT_POSITION: Item<Position> = Item::new("next_position");

// Helper function to get next available position
pub fn get_next_position(current: Position) -> Option<Position> {
    let mut next = current;
    
    // Move to next position
    next.x += 1;
    if next.x > MAX_POSITION {
        next.x = 0;
        next.y += 1;
        if next.y > MAX_POSITION {
            return None; // No more positions available
        }
    }
    
    Some(next)
}

// Helper function to validate position
pub fn validate_position(position: &Position) -> bool {
    position.x <= MAX_POSITION && position.y <= MAX_POSITION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_position() {
        // Test normal increment
        let pos = Position { x: 5, y: 5 };
        let next = get_next_position(pos).unwrap();
        assert_eq!(next.x, 6);
        assert_eq!(next.y, 5);

        // Test row wrap
        let pos = Position { x: MAX_POSITION, y: 5 };
        let next = get_next_position(pos).unwrap();
        assert_eq!(next.x, 0);
        assert_eq!(next.y, 6);

        // Test grid end
        let pos = Position { x: MAX_POSITION, y: MAX_POSITION };
        assert!(get_next_position(pos).is_none());
    }

    #[test]
    fn test_validate_position() {
        // Valid position
        assert!(validate_position(&Position { x: 0, y: 0 }));
        assert!(validate_position(&Position { x: MAX_POSITION, y: MAX_POSITION }));

        // Invalid position
        assert!(!validate_position(&Position { x: MAX_POSITION + 1, y: 0 }));
        assert!(!validate_position(&Position { x: 0, y: MAX_POSITION + 1 }));
    }
} 