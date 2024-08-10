use serde::Serialize;
use reqwest::Client;
use std::env;
use eyre::Result;

#[derive(Serialize)]
pub struct BlockData {
    pub block_number: u64,
    pub block_hash: String,
    pub parent_hash: String,
    pub nonce: String,
    pub logs_bloom: String,
    pub transactions_root: String,
    pub state_root: String,
    pub receipts_root: String,
    pub miner: String,
    pub difficulty: String,
    pub total_difficulty: Option<String>,
    pub extra_data: Option<String>,
    pub gas_limit: u128,
    pub gas_used: u128,
}

pub struct SupabaseClient {
    client: Client,
    url: String,
    api_key: String,
}

impl SupabaseClient {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
        let api_key = env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set");

        SupabaseClient {
            client: Client::new(),
            url,
            api_key,
        }
    }

    pub async fn store_block_data(&self, block_data: BlockData) -> Result<()> {
        let url = format!("{}/rest/v1/blocks", self.url);
        let response = self.client
            .post(&url)
            .header("apikey", &self.api_key)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .header("Content-Type", "application/json")
            .json(&block_data)
            .send()
            .await?;

        if response.status().is_success() {
            println!("Block data stored successfully.");
        } else {
            println!("Failed to store block data: {:?}", response.text().await?);
        }

        Ok(())
    }
}
