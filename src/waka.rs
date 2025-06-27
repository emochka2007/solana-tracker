use crate::waka_types::ApiResponse;
use anyhow::anyhow;
use reqwest::Client;
use std::env;
use tracing::error;

pub struct WakaClient {
    key: String,
    base_url: String,
}

impl WakaClient {
    pub fn new() -> Self {
        Self {
            key: env::var("WAKA_KEY").unwrap(),
            base_url: "https://wakatime.com/api/v1".to_string(),
        }
    }

    pub async fn get_activity_last_day(&self) -> anyhow::Result<u64> {
        let client = Client::new();
        let url = format!("{}/users/current/summaries", &self.base_url);
        let key_query = ("api_key", self.key.as_str());
        let range_query = ("range", "yesterday");
        let response = client
            .get(url)
            .query(&[key_query, range_query])
            .send()
            .await;
        let api_response = match response {
            Ok(response) => response.json::<ApiResponse>().await,
            Err(e) => {
                error!("{:?}", e);
                return Err(anyhow!("User summaries error {:?}", e));
            }
        }?;
        Ok(api_response.cumulative_total.seconds as u64)
    }
}
