use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sg721::ExecuteMsg as Sg721ExecuteMsg;
use cosmwasm_std::Empty;
use crate::types::Extension;

/// Pixel update information
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PixelUpdate {
    /// The ID of the pixel to update
    pub pixel_id: u32,
    /// The new RGB color for the pixel
    pub color: [u8; 3],
    /// The expiration timestamp for this update
    pub expiration: u64,
}

/// Execute messages
/// This contract extends sg721-base by adding pixel color functionality
/// All standard NFT operations are forwarded to sg721-base through the Base variant
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Standard sg721-base NFT functionality
    Base(Sg721ExecuteMsg<Extension, Empty>),
    
    /// Custom extension: Set pixel color
    /// Flow:
    /// 1. Verify hash(current_tile_metadata) matches stored hash
    /// 2. Verify pixel availability based on expiration
    /// 3. Apply update if available
    /// 4. Calculate and store new hash(tile_metadata)
    SetPixelColor {
        /// Current tile metadata for verification
        current_tile_metadata: Vec<u8>,
        /// The pixel update to apply
        pixel_update: PixelUpdate,
    },
}

// Re-export base messages
pub use sg721::InstantiateMsg;
pub use sg721_base::msg::QueryMsg;
  