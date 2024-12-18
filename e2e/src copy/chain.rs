use crate::error::Error;
use serde_json::Value;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub struct Chain {
    node: String,
    chain_id: String,
}

impl Chain {
    pub fn new(node: &str, chain_id: &str) -> Self {
        Self {
            node: node.to_string(),
            chain_id: chain_id.to_string(),
        }
    }

    pub fn get_node(&self) -> &str {
        &self.node
    }

    pub async fn execute_tx(
        &self,
        contract_addr: &str,
        msg: &Value,
        from: &str,
        funds: Option<&str>,
    ) -> Result<Value, Error> {
        let msg_str = msg.to_string();
        let mut args = vec![
            "tx",
            "wasm",
            "execute",
            contract_addr,
            &msg_str,
            "--from",
            from,
            "--chain-id",
            &self.chain_id,
            "--node",
            &self.node,
            "--gas",
            "auto",
            "--gas-adjustment",
            "1.3",
            "--fees",
            "2000ustars",
            "--output",
            "json",
            "--keyring-backend",
            "test",
            "-y",
            "--broadcast-mode",
            "sync",
        ];

        if let Some(funds) = funds {
            args.push("--amount");
            args.push(funds);
        }

        println!("Executing command: starsd {}", args.join(" "));
        let output = Command::new("starsd")
            .args(&args)
            .output()
            .map_err(|e| Error::CommandExecution(e.to_string()))?;

        if !output.status.success() {
            return Err(Error::TransactionFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let tx_response: Value = serde_json::from_slice(&output.stdout)?;
        let txhash = tx_response["txhash"]
            .as_str()
            .ok_or_else(|| Error::TransactionFailed("No txhash in response".to_string()))?;

        println!("Transaction hash: {}", txhash);
        println!("Waiting for transaction to be included in a block...");

        // Wait for transaction to be included in a block with retries
        let mut retries = 0;
        let max_retries = 10;
        let mut last_error = None;

        while retries < max_retries {
            thread::sleep(Duration::from_secs(6));

            let output = Command::new("starsd")
                .args([
                    "query", "tx", txhash, "--node", &self.node, "--output", "json",
                ])
                .output()
                .map_err(|e| Error::CommandExecution(e.to_string()))?;

            if output.status.success() {
                let tx_result: Value = serde_json::from_slice(&output.stdout)?;
                println!("Transaction result: {}", tx_result);

                if tx_result["code"].as_u64().unwrap_or(1) != 0 {
                    return Err(Error::TransactionFailed(
                        tx_result["raw_log"]
                            .as_str()
                            .unwrap_or("Unknown error")
                            .to_string(),
                    ));
                }

                return Ok(tx_result);
            }

            last_error = Some(String::from_utf8_lossy(&output.stderr).to_string());
            retries += 1;
            println!("Retry {} of {}", retries, max_retries);
        }

        Err(Error::TransactionFailed(format!(
            "Transaction not found after {} retries. Last error: {}",
            max_retries,
            last_error.unwrap_or_else(|| "Unknown error".to_string())
        )))
    }

    pub async fn query(&self, contract_addr: &str, msg: &Value) -> Result<Value, Error> {
        let msg_str = msg.to_string();
        println!(
            "Querying contract {} with message: {}",
            contract_addr, msg_str
        );
        let output = Command::new("starsd")
            .args([
                "query",
                "wasm",
                "contract-state",
                "smart",
                contract_addr,
                &msg_str,
                "--output",
                "json",
                "--node",
                &self.node,
            ])
            .output()
            .map_err(|e| Error::CommandExecution(e.to_string()))?;

        if !output.status.success() {
            return Err(Error::QueryFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let response: Value = serde_json::from_slice(&output.stdout)?;
        println!("Query response: {}", response);
        Ok(response["data"].clone())
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self {
            node: "https://rpc.elgafar-1.stargaze-apis.com:443".to_string(),
            chain_id: "elgafar-1".to_string(),
        }
    }
}
