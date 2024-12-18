use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub contract_address: String,
    pub cosmos_rpc: String,
    pub ipfs_api: String,
}

impl Config {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_str = tokio::fs::read_to_string(path).await?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }
} 