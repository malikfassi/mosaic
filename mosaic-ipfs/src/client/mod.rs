use anyhow::Result;
use reqwest::Client as HttpClient;
use crate::service::Config;

pub struct Client {
    config: Config,
    http: HttpClient,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            http: HttpClient::new(),
        }
    }

    pub async fn pin_file(&self, data: &[u8]) -> Result<String> {
        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(data.to_vec()));

        let response = self.http
            .post(format!("{}/api/v0/add", self.config.ipfs_api))
            .multipart(form)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let hash = response["Hash"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("No hash in response"))?;

        Ok(hash.to_string())
    }
} 