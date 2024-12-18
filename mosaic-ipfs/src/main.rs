mod config;
mod ipfs;
mod listener;

use clap::Parser;
use tracing::{info, error};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Parse command line arguments
    let args = config::CliArgs::parse();

    // Load configuration
    let config = config::Config::new(&args)?;
    info!("Configuration loaded");

    // Create and start chain listener
    let mut listener = listener::ChainListener::new(
        &config.cosmos_rpc,
        None, // WebSocket URL if needed
        &config.contract_address,
        &config.ipfs_api,
    ).await?;

    info!("Starting chain listener...");
    
    // Start listening for events
    if let Err(e) = listener.start().await {
        error!("Error in chain listener: {}", e);
    }

    Ok(())
} 