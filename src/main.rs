mod db;
mod init;

use alloy::providers::{Provider};
use futures_util::StreamExt;
use dotenv::dotenv;
use std::env;
use alloy::eips::BlockId;
use alloy::rpc::types::BlockTransactionsKind;
use db::{BlockData, SupabaseClient};
use init::{init_provider};


#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    sync_older_blocks().await?;

    Ok(())
}


// This function listens to and stores the latest block details
async fn listen_new_blocks() -> eyre::Result<()> {
    let provider = init_provider().await?;

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
            gas_used: block.header.gas_used,
        };

        supabase_client.store_block_data(block_data).await?;
    }

    Ok(())
}

async fn sync_older_blocks() -> eyre::Result<()> {
    let provider = init_provider().await?;
    let supabase_client = SupabaseClient::new();

    let start_block: u64 = env::var("START_BLOCK")
        .expect("Start block not found")
        .parse()
        .expect("START_BLOCK must be a valid unsigned integer");
    let end_block: u64 = env::var("END_BLOCK")
        .expect("End block not found")
        .parse()
        .expect("END_BLOCK must be a valid unsigned integer");

    for block_number in start_block..=end_block {
        // Convert block_number to BlockId and choose the transaction kind
        let block_id = BlockId::Number(block_number.into());
        let transactions_kind = BlockTransactionsKind::Full;

        if let Some(block) = provider.get_block(block_id, transactions_kind).await? {
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
                gas_used: block.header.gas_used,
            };

            supabase_client.store_block_data(block_data).await?;
            println!("Synced block: {}", block_number);
        } else {
            println!("Block {} not found", block_number);
        }
    }

    Ok(())
}
