use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::constants::tile;

/// Number of pixels per tile
pub fn pixels_per_tile() -> u32 {
    tile::total_pixels_per_tile()
}

/// Extension for sg721-base token metadata
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Extension {
    /// Current tile metadata
    pub tile_metadata: Vec<u8>,
}

/// Metadata for a single pixel
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PixelMetadata {
    /// RGB color values
    pub color: [u8; 3],
    /// Expiration timestamp
    pub expiration: u64,
    /// Unique pixel identifier
    pub pixel_id: u32,
}

/// Metadata for a tile containing multiple pixels
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TileMetadata {
    /// Unique tile identifier
    pub tile_id: u32,
    /// Array of pixel metadata
    pub pixels: Vec<PixelMetadata>,
}

impl TileMetadata {
    /// Create a new tile with default white pixels
    pub fn new(tile_id: u32, _timestamp: u64) -> Self {
        let pixels_per_tile = pixels_per_tile();
        let mut pixels = Vec::with_capacity(pixels_per_tile as usize);
        for pixel_index in 0..pixels_per_tile {
            pixels.push(PixelMetadata {
                color: [255, 255, 255], // white
                expiration: 0,          // never expired
                pixel_id: tile_id * pixels_per_tile + pixel_index,
            });
        }
        Self { tile_id, pixels }
    }

    /// Convert tile metadata to bytes for storage
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    /// Create tile metadata from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data)
    }

    /// Calculate hash of tile metadata
    pub fn calculate_hash(&self) -> [u8; 32] {
        let serialized = self.to_bytes();
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        hasher.finalize().into()
    }

    /// Update a pixel's color and expiration
    pub fn update_pixel(
        &mut self,
        pixel_id: u32,
        color: [u8; 3],
        expiration: u64,
    ) -> Result<(), String> {
        if !is_pixel_in_tile(pixel_id, self.tile_id) {
            return Err("Pixel ID out of tile range".to_string());
        }

        let pixel_index = get_pixel_index(pixel_id);
        let pixel = &mut self.pixels[pixel_index];
        
        pixel.color = color;
        pixel.expiration = expiration;
        
        Ok(())
    }

    /// Get a pixel's metadata
    pub fn get_pixel(&self, pixel_id: u32) -> Result<&PixelMetadata, String> {
        if !is_pixel_in_tile(pixel_id, self.tile_id) {
            return Err("Pixel ID out of tile range".to_string());
        }

        let pixel_index = get_pixel_index(pixel_id);
        Ok(&self.pixels[pixel_index])
    }
}

/// Helper function to check if a pixel belongs to a tile
pub fn is_pixel_in_tile(pixel_id: u32, tile_id: u32) -> bool {
    let pixels_per_tile = pixels_per_tile();
    let tile_start = tile_id * pixels_per_tile;
    let tile_end = tile_start + pixels_per_tile;
    pixel_id >= tile_start && pixel_id < tile_end
}

/// Helper function to get the index of a pixel within its tile
pub fn get_pixel_index(pixel_id: u32) -> usize {
    (pixel_id % pixels_per_tile()) as usize
}
