use crate::PixelExtension;
use cosmwasm_std::Empty;
use cw_storage_plus::Map;
use sg721_base::Sg721Contract;

pub type Sg721PixelContract = Sg721Contract<PixelExtension>;

// Additional state for pixel-specific functionality
pub const PIXEL_COORDINATES: Map<(u32, u32), String> = Map::new("pixel_coordinates");

// Constants for pixel grid
pub const MAX_X: u32 = 1000;
pub const MAX_Y: u32 = 1000; 