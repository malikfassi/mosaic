// Test addresses
pub const CREATOR: &str = "creator";
pub const MINTER: &str = "minter";
pub const OWNER: &str = "owner";
pub const HACKER: &str = "hacker";
pub const USER1: &str = "user1";
pub const USER2: &str = "user2";

// Test token IDs
pub const TOKEN1: &str = "tile1";
pub const TOKEN2: &str = "tile2";
pub const TOKEN3: &str = "tile3";

// Test collection info
pub const COLLECTION_NAME: &str = "MosaicTiles";
pub const COLLECTION_SYMBOL: &str = "TILE";
pub const COLLECTION_DESCRIPTION: &str = "Mosaic Tile NFTs";
pub const COLLECTION_IMAGE: &str = "https://example.com/image.png";

// Test colors
pub const RED: (u8, u8, u8) = (255, 0, 0);
pub const GREEN: (u8, u8, u8) = (0, 255, 0);
pub const BLUE: (u8, u8, u8) = (0, 0, 255);
pub const WHITE: (u8, u8, u8) = (255, 255, 255);
pub const BLACK: (u8, u8, u8) = (0, 0, 0);

// Test positions
pub const POSITION1: (u32, u32) = (1, 1);
pub const POSITION2: (u32, u32) = (2, 2);
pub const POSITION3: (u32, u32) = (3, 3);
pub const INVALID_POSITION: (u32, u32) = (100, 100); // Beyond MAX_POSITION 