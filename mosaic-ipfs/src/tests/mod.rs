pub mod service {
    pub mod config;
    pub mod ipfs;
    pub mod listener;
    pub mod metadata;
}

pub mod client {
    pub mod api;
    pub mod storage;
    pub mod pinning;
}

pub mod integration {
    pub mod event_handling;
    pub mod metadata_sync;
}

// Re-export test utilities
mod test_utils;
pub use test_utils::*;

// Common test utilities
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;

pub async fn setup_test_ipfs() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    // Create test config
    let config = r#"{
        "mosaic": {
            "services": {
                "ipfs": {
                    "api_endpoint": "http://localhost:5001",
                    "gateway": "http://localhost:8080",
                    "pin_policy": {
                        "enabled": true,
                        "max_size": 1048576,
                        "retention_period": 2592000
                    }
                },
                "chain": {
                    "rpc_endpoint": "http://localhost:26657",
                    "ws_endpoint": "ws://localhost:26657/websocket",
                    "chain_id": "test-1",
                    "gas_price": {
                        "amount": "0.025",
                        "denom": "ustars"
                    }
                }
            }
        }
    }"#;

    fs::write(&config_path, config).await.unwrap();
    (temp_dir, config_path)
}

pub fn create_test_metadata() -> serde_json::Value {
    serde_json::json!({
        "color": {
            "r": 255,
            "g": 0,
            "b": 0
        },
        "expiration": null,
        "version": 1,
        "previous_hash": null
    })
} 