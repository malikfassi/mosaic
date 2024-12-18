pub mod scenarios {
    pub mod pixel_lifecycle;
    pub mod tile_management;
    pub mod fee_distribution;
    pub mod metadata_sync;
}

pub mod performance {
    pub mod batch_operations;
    pub mod concurrent_updates;
    pub mod storage_scaling;
}

pub mod integration {
    pub mod contract_ipfs;
    pub mod chain_events;
    pub mod metadata_validation;
}

// Re-export test utilities
mod test_utils;
pub use test_utils::*;

// Common test utilities
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use mosaic_contract::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use mosaic_ipfs::service::Config;

pub async fn setup_test_environment() -> (Config, String) {
    // Setup temporary test environment
    let (temp_dir, config_path) = mosaic_ipfs::tests::setup_test_ipfs().await;
    
    // Return config and temp dir path
    let config = Config::load(&config_path).await.unwrap();
    let temp_path = temp_dir.path().to_str().unwrap().to_string();
    
    (config, temp_path)
} 