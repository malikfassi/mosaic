use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to root config file
    #[arg(short, long, default_value = "config.json")]
    pub config: PathBuf,

    /// Override Cosmos RPC endpoint
    #[arg(long)]
    pub cosmos_rpc: Option<String>,

    /// Override IPFS API endpoint
    #[arg(long)]
    pub ipfs_api: Option<String>,

    /// Override contract address
    #[arg(long)]
    pub contract_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub cosmos_rpc: String,
    pub ipfs_api: String,
    pub ipfs_gateway: String,
    pub contract_address: String,
    pub chain_id: String,
    pub start_height: Option<u64>,
    pub poll_interval: Option<u64>,
    pub batch_size: Option<u32>,
    pub pin_policy: PinPolicy,
}

#[derive(Debug, Deserialize)]
pub struct PinPolicy {
    pub enabled: bool,
    pub max_size: u64,
    pub retention_period: u64,
}

#[derive(Debug, Deserialize)]
pub struct RootConfig {
    pub mosaic: MosaicConfig,
}

#[derive(Debug, Deserialize)]
pub struct MosaicConfig {
    pub fees: FeeConfig,
    pub tile: TileConfig,
    pub services: ServicesConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServicesConfig {
    pub ipfs: IpfsConfig,
    pub chain: ChainConfig,
    pub indexer: IndexerConfig,
}

#[derive(Debug, Deserialize)]
pub struct IpfsConfig {
    pub api_endpoint: String,
    pub gateway: String,
    pub pin_policy: PinPolicyConfig,
}

#[derive(Debug, Deserialize)]
pub struct PinPolicyConfig {
    pub enabled: bool,
    pub max_size: u64,
    pub retention_period: u64,
}

#[derive(Debug, Deserialize)]
pub struct ChainConfig {
    pub rpc_endpoint: String,
    pub ws_endpoint: String,
    pub chain_id: String,
    pub gas_price: GasPrice,
}

#[derive(Debug, Deserialize)]
pub struct GasPrice {
    pub amount: String,
    pub denom: String,
}

#[derive(Debug, Deserialize)]
pub struct IndexerConfig {
    pub enabled: bool,
    pub batch_size: u32,
    pub poll_interval: u64,
    pub start_height: u64,
}

#[derive(Debug, Deserialize)]
pub struct FeeConfig {
    pub base_fee: CoinConfig,
    pub developer_fee_percent: u8,
    pub mint_price: CoinConfig,
}

#[derive(Debug, Deserialize)]
pub struct TileConfig {
    pub max_count: u32,
    pub size: u32,
}

#[derive(Debug, Deserialize)]
pub struct CoinConfig {
    pub amount: String,
    pub denom: String,
}

impl Config {
    pub fn new(args: &CliArgs) -> anyhow::Result<Self> {
        // First, try to find root config relative to current dir
        let root_config_path = if args.config.exists() {
            args.config.clone()
        } else {
            // Try to find it in parent directories
            let mut current = std::env::current_dir()?;
            let mut config_path = None;
            while current.parent().is_some() {
                let test_path = current.join("config.json");
                if test_path.exists() {
                    config_path = Some(test_path);
                    break;
                }
                current = current.parent().unwrap().to_path_buf();
            }
            config_path.ok_or_else(|| anyhow::anyhow!("Could not find root config.json"))?
        };

        // Load root config
        let root_content = std::fs::read_to_string(&root_config_path)?;
        let root_config: RootConfig = serde_json::from_str(&root_content)?;

        // Create service config from root config
        let services = &root_config.mosaic.services;
        let mut config = Config {
            cosmos_rpc: services.chain.rpc_endpoint.clone(),
            ipfs_api: services.ipfs.api_endpoint.clone(),
            ipfs_gateway: services.ipfs.gateway.clone(),
            contract_address: std::env::var("CONTRACT_ADDRESS").unwrap_or_default(),
            chain_id: services.chain.chain_id.clone(),
            start_height: Some(services.indexer.start_height),
            poll_interval: Some(services.indexer.poll_interval),
            batch_size: Some(services.indexer.batch_size),
            pin_policy: PinPolicy {
                enabled: services.ipfs.pin_policy.enabled,
                max_size: services.ipfs.pin_policy.max_size,
                retention_period: services.ipfs.pin_policy.retention_period,
            },
        };

        // Override with CLI args
        if let Some(rpc) = &args.cosmos_rpc {
            config.cosmos_rpc = rpc.clone();
        }
        if let Some(api) = &args.ipfs_api {
            config.ipfs_api = api.clone();
        }
        if let Some(addr) = &args.contract_address {
            config.contract_address = addr.clone();
        }

        // Validate
        if config.contract_address.is_empty() {
            anyhow::bail!("Contract address is required (set via --contract-address or CONTRACT_ADDRESS env var)");
        }

        Ok(config)
    }
} 