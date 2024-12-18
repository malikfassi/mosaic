use anyhow::Result;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient, TryFromUri};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelMetadata {
    pub color: Color,
    pub version: u32,
    pub previous_hash: Option<String>,
    pub timestamp: u64,
    pub expiration: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct IpfsService {
    client: IpfsClient,
    cache: HashMap<String, String>, // pixel_id -> ipfs_hash
}

impl IpfsService {
    pub fn new(api_url: &str) -> Result<Self> {
        let client = IpfsClient::from_str(api_url)?;
        Ok(Self {
            client,
            cache: HashMap::new(),
        })
    }

    pub async fn store_metadata(&mut self, pixel_id: &str, metadata: &PixelMetadata) -> Result<String> {
        // Serialize metadata
        let json = serde_json::to_vec(metadata)?;
        
        // Store in IPFS
        let res = self.client.add(json).await?;
        let ipfs_hash = res.hash;

        debug!("Stored metadata for pixel {} with hash {}", pixel_id, ipfs_hash);
        
        // Update cache
        self.cache.insert(pixel_id.to_string(), ipfs_hash.clone());
        
        Ok(ipfs_hash)
    }

    pub async fn get_metadata(&self, ipfs_hash: &str) -> Result<PixelMetadata> {
        // Get from IPFS
        let data = self.client.cat(ipfs_hash).await?;
        
        // Deserialize
        let metadata = serde_json::from_slice(&data)?;
        Ok(metadata)
    }

    pub async fn verify_metadata(&self, ipfs_hash: &str, expected_hash: &str) -> Result<bool> {
        // Get metadata
        let metadata = self.get_metadata(ipfs_hash).await?;
        
        // Calculate hash
        let mut hasher = sha2::Sha256::new();
        hasher.update(b"pixel_v1_");
        hasher.update(serde_json::to_vec(&metadata)?);
        let calculated_hash = hex::encode(hasher.finalize());
        
        Ok(calculated_hash == expected_hash)
    }

    pub fn get_cached_hash(&self, pixel_id: &str) -> Option<&String> {
        self.cache.get(pixel_id)
    }

    pub async fn pin_hash(&self, ipfs_hash: &str) -> Result<()> {
        self.client.pin_add(ipfs_hash, false).await?;
        debug!("Pinned hash {}", ipfs_hash);
        Ok(())
    }

    pub async fn unpin_hash(&self, ipfs_hash: &str) -> Result<()> {
        self.client.pin_rm(ipfs_hash, false).await?;
        debug!("Unpinned hash {}", ipfs_hash);
        Ok(())
    }
} 