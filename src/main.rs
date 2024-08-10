mod db;

use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use futures_util::StreamExt;
use dotenv::dotenv;
use std::env;
use db::{BlockData, SupabaseClient};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    listen_blocks().await?;

    Ok(())
}

// This function listens to and stores the latest block details
async fn listen_blocks() -> eyre::Result<()> {
    let ws = WsConnect::new(env::var("WSS_URL").expect("WSS_URL environment variable is not set"));
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    let subscription = provider.subscribe_blocks().await?;
    let mut stream = subscription.into_stream();
    let supabase_client = SupabaseClient::new();

    while let Some(block) = stream.next().await {

        let block_number = block.header.number.expect("Block number is None");
        println!("Received block:{:#?}",block_number);

        let block_data = BlockData {
            block_number,
            block_hash: format!("{:?}", block.header.hash.unwrap_or_default()),
            parent_hash: format!("{:?}", block.header.parent_hash),
            nonce: format!("{:?}", block.header.nonce.unwrap_or_default()),
            logs_bloom: format!("{:?}", block.header.logs_bloom),
            transactions_root: format!("{:?}", block.header.transactions_root),
            state_root: format!("{:?}", block.header.state_root),
            receipts_root: format!("{:?}", block.header.receipts_root),
            miner: format!("{:?}", block.header.miner),
            difficulty: format!("{}", block.header.difficulty),
            total_difficulty: block.header.total_difficulty.map(|d| format!("{:?}", d)),
            extra_data: Some(format!("{:?}", block.header.extra_data)),
            gas_limit: block.header.gas_limit,
            gas_used: block.header.gas_used ,
        };

        supabase_client.store_block_data(block_data).await?;
    }

    Ok(())
}
