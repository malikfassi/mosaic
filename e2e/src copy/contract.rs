use crate::chain::Chain;
use crate::error::Error;
use serde_json::{json, Value};
use std::fs;

pub struct Contract {
    pub address: String,
    pub chain: Chain,
}

impl Contract {
    pub fn new(address: &str, chain: Chain) -> Self {
        Self {
            address: address.to_string(),
            chain,
        }
    }

    pub fn from_env(env_var: &str) -> Result<Self, Error> {
        let address = std::env::var(env_var)?;
        Ok(Self {
            address,
            chain: Chain::default(),
        })
    }

    pub async fn execute(
        &self,
        msg: &Value,
        from: &str,
        funds: Option<&str>,
    ) -> Result<Value, Error> {
        self.chain.execute_tx(&self.address, msg, from, funds).await
    }

    pub async fn query(&self, msg: &Value) -> Result<Value, Error> {
        self.chain.query(&self.address, msg).await
    }

    pub fn validate_message(&self, msg: &Value, schema_file: &str) -> Result<(), Error> {
        let schema_path = format!("schema/{}", schema_file);
        let schema_str = fs::read_to_string(&schema_path)
            .map_err(|e| Error::SchemaValidation(format!("Failed to read schema file: {}", e)))?;

        let schema: Value = serde_json::from_str(&schema_str)
            .map_err(|e| Error::SchemaValidation(format!("Failed to parse schema: {}", e)))?;

        if let Some(required) = schema["required"].as_array() {
            for field in required {
                if let Some(field_name) = field.as_str() {
                    if msg.get(field_name).is_none() {
                        return Err(Error::SchemaValidation(format!(
                            "Missing required field: {}",
                            field_name
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn query_balance(&self, address: &str, denom: &str) -> Result<String, Error> {
        let output = std::process::Command::new("starsd")
            .args([
                "query",
                "bank",
                "balances",
                address,
                "--denom",
                denom,
                "--node",
                self.chain.get_node(),
                "--output",
                "json",
            ])
            .output()
            .map_err(|e| Error::CommandExecution(e.to_string()))?;

        if !output.status.success() {
            return Err(Error::QueryFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let response: Value = serde_json::from_slice(&output.stdout)?;
        let amount = response["amount"]
            .as_str()
            .ok_or_else(|| Error::QueryFailed("Amount not found in response".to_string()))?;

        Ok(amount.to_string())
    }
}

// Helper trait for CW721 contracts
#[async_trait::async_trait]
pub trait CW721Contract {
    async fn execute(&self, msg: &Value, from: &str, funds: Option<&str>) -> Result<Value, Error>;
    async fn query(&self, msg: &Value) -> Result<Value, Error>;

    async fn mint(
        &self,
        token_id: &str,
        owner: &str,
        token_uri: Option<&str>,
        extension: Option<Value>,
        from: &str,
    ) -> Result<Value, Error>;

    async fn transfer(&self, token_id: &str, recipient: &str, from: &str) -> Result<Value, Error>;

    async fn query_owner_of(&self, token_id: &str) -> Result<String, Error>;

    async fn query_token_info(&self, token_id: &str) -> Result<Value, Error>;

    async fn query_contract_info(&self) -> Result<Value, Error>;
}

#[async_trait::async_trait]
impl CW721Contract for Contract {
    async fn execute(&self, msg: &Value, from: &str, funds: Option<&str>) -> Result<Value, Error> {
        self.chain.execute_tx(&self.address, msg, from, funds).await
    }

    async fn query(&self, msg: &Value) -> Result<Value, Error> {
        self.chain.query(&self.address, msg).await
    }

    async fn mint(
        &self,
        token_id: &str,
        owner: &str,
        token_uri: Option<&str>,
        extension: Option<Value>,
        from: &str,
    ) -> Result<Value, Error> {
        let msg = json!({
            "cw721": {
                "mint": {
                    "token_id": token_id,
                    "owner": owner,
                    "token_uri": token_uri,
                    "extension": extension.unwrap_or(json!({}))
                }
            }
        });

        self.execute(&msg, from, None).await
    }

    async fn transfer(&self, token_id: &str, recipient: &str, from: &str) -> Result<Value, Error> {
        let msg = json!({
            "cw721": {
                "transfer_nft": {
                    "token_id": token_id,
                    "recipient": recipient
                }
            }
        });

        self.execute(&msg, from, None).await
    }

    async fn query_owner_of(&self, token_id: &str) -> Result<String, Error> {
        let msg = json!({
            "cw721": {
                "owner_of": {
                    "token_id": token_id
                }
            }
        });

        let response = self.query(&msg).await?;
        response["owner"]
            .as_str()
            .ok_or_else(|| Error::QueryFailed("Owner not found in response".to_string()))
            .map(String::from)
    }

    async fn query_token_info(&self, token_id: &str) -> Result<Value, Error> {
        let msg = json!({
            "cw721": {
                "nft_info": {
                    "token_id": token_id
                }
            }
        });

        self.query(&msg).await
    }

    async fn query_contract_info(&self) -> Result<Value, Error> {
        let msg = json!({
            "cw721": {
                "contract_info": {}
            }
        });

        self.query(&msg).await
    }
}

// Helper trait for Mosaic Tile NFT contract
#[async_trait::async_trait]
pub trait MosaicTileContract: CW721Contract {
    async fn mint_tile(&self, tile_id: u32, owner: &str, from: &str) -> Result<Value, Error> {
        let msg = json!({
            "mint_tile": {
                "tile_id": tile_id,
                "owner": owner
            }
        });

        self.execute(&msg, from, None).await
    }

    async fn set_pixel_color(
        &self,
        pixel_id: u32,
        color: (u8, u8, u8),
        from: &str,
    ) -> Result<Value, Error> {
        let msg = json!({
            "set_pixel_color": {
                "pixel_id": pixel_id,
                "color": {
                    "r": color.0,
                    "g": color.1,
                    "b": color.2
                }
            }
        });

        self.execute(&msg, from, None).await
    }

    async fn query_tile_state(&self, tile_id: u32) -> Result<Value, Error> {
        let msg = json!({
            "tile_state": {
                "tile_id": tile_id
            }
        });

        self.query(&msg).await
    }

    async fn query_pixel_state(&self, pixel_id: u32) -> Result<Value, Error> {
        let msg = json!({
            "pixel_state": {
                "pixel_id": pixel_id
            }
        });

        self.query(&msg).await
    }
}

// Implement MosaicTileContract for any Contract
impl<T: CW721Contract> MosaicTileContract for T {}
