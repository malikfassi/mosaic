use anyhow::Result;
use tendermint_rpc::{Client, HttpClient, SubscriptionClient, WebSocketClient};
use tendermint_rpc::query::EventType;
use tendermint_rpc::event::Event;
use std::collections::HashMap;
use tracing::{debug, error, info};
use crate::ipfs::{IpfsService, PixelMetadata, Color};

const EVENT_TYPE_PIXEL_UPDATE: &str = "pixel_update";

pub struct ChainListener {
    rpc_client: HttpClient,
    ws_client: Option<WebSocketClient>,
    contract_address: String,
    last_height: u64,
    ipfs: IpfsService,
}

impl ChainListener {
    pub async fn new(
        rpc_url: &str,
        ws_url: Option<&str>,
        contract_address: &str,
        ipfs_url: &str,
    ) -> Result<Self> {
        let rpc_client = HttpClient::new(rpc_url)?;
        let ws_client = if let Some(url) = ws_url {
            Some(WebSocketClient::new(url).await?)
        } else {
            None
        };

        Ok(Self {
            rpc_client,
            ws_client,
            contract_address: contract_address.to_string(),
            last_height: 0,
            ipfs: IpfsService::new(ipfs_url)?,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        // Subscribe to events if websocket is available
        if let Some(ws) = &mut self.ws_client {
            self.subscribe_to_events(ws).await?;
        } else {
            self.poll_events().await?;
        }
        Ok(())
    }

    async fn subscribe_to_events(&mut self, ws: &mut WebSocketClient) -> Result<()> {
        let query = format!(
            "tm.event='Tx' AND wasm.contract_address='{}' AND wasm._contract_address='{}'",
            self.contract_address, self.contract_address
        );

        let mut subscription = ws.subscribe(query).await?;
        info!("Subscribed to events");

        while let Some(event) = subscription.next().await {
            match event {
                Ok(event) => self.handle_event(event).await?,
                Err(e) => error!("Error receiving event: {}", e),
            }
        }

        Ok(())
    }

    async fn poll_events(&mut self) -> Result<()> {
        loop {
            // Get latest block
            let status = self.rpc_client.status().await?;
            let current_height = status.sync_info.latest_block_height.value();

            if current_height > self.last_height {
                // Query events
                let query = format!(
                    "tx.height>={} AND wasm.contract_address='{}' AND wasm._contract_address='{}'",
                    self.last_height + 1,
                    self.contract_address,
                    self.contract_address
                );

                let results = self.rpc_client.tx_search(query, false, 1, 100, "").await?;
                
                for tx in results.txs {
                    for event in tx.result.events {
                        self.handle_event(event).await?;
                    }
                }

                self.last_height = current_height;
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    async fn handle_event(&mut self, event: Event) -> Result<()> {
        if event.type_str != EVENT_TYPE_PIXEL_UPDATE {
            return Ok(());
        }

        let attrs: HashMap<_, _> = event.attributes
            .into_iter()
            .map(|attr| (attr.key, attr.value))
            .collect();

        // Extract metadata
        let metadata = self.extract_metadata(&attrs)?;
        let pixel_id = attrs.get("pixel_id")
            .ok_or_else(|| anyhow::anyhow!("No pixel_id in event"))?;
        let metadata_hash = attrs.get("metadata_hash")
            .ok_or_else(|| anyhow::anyhow!("No metadata_hash in event"))?;

        // Store in IPFS
        let ipfs_hash = self.ipfs.store_metadata(pixel_id, &metadata).await?;
        
        // Verify hash
        if !self.ipfs.verify_metadata(&ipfs_hash, metadata_hash).await? {
            error!("Metadata hash verification failed for pixel {}", pixel_id);
            return Ok(());
        }

        // Pin the content
        self.ipfs.pin_hash(&ipfs_hash).await?;

        info!(
            "Processed pixel update: pixel_id={}, ipfs_hash={}, version={}",
            pixel_id, ipfs_hash, metadata.version
        );

        Ok(())
    }

    fn extract_metadata(&self, attrs: &HashMap<String, String>) -> Result<PixelMetadata> {
        Ok(PixelMetadata {
            color: Color {
                r: attrs.get("color_r").ok_or_else(|| anyhow::anyhow!("No color_r"))?.parse()?,
                g: attrs.get("color_g").ok_or_else(|| anyhow::anyhow!("No color_g"))?.parse()?,
                b: attrs.get("color_b").ok_or_else(|| anyhow::anyhow!("No color_b"))?.parse()?,
            },
            version: attrs.get("version").ok_or_else(|| anyhow::anyhow!("No version"))?.parse()?,
            previous_hash: attrs.get("previous_hash").cloned(),
            timestamp: attrs.get("timestamp").ok_or_else(|| anyhow::anyhow!("No timestamp"))?.parse()?,
            expiration: attrs.get("expiration").and_then(|e| e.parse().ok()),
        })
    }
} 