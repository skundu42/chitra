mod db;
mod init;

use alloy::providers::{Provider};
use futures_util::StreamExt;
use dotenv::dotenv;
use std::env;
use alloy::eips::BlockId;
<<<<<<< HEAD
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
=======
use alloy::rpc::types::{BlockTransactions, BlockTransactionsKind};
use db::{BlockData, SupabaseClient, TransactionData};
use init::{init_provider};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();
    
    //To: Create logic according to user parameters to handle old and new sync
    
    sync_older_blocks().await?;
    //listen_new_blocks().await?;
>>>>>>> dev

    Ok(())
}

// This function listens to and stores the latest block details and their transactions
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
        println!("Stored block data for block: {}", block_number);

        // Fetch and store transactions for this block
        sync_tx_data(block_number).await?;
    }

    Ok(())
}

// This function syncs older blocks and their transactions
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

            // Fetch and store transactions for this block
            sync_tx_data(block_number).await?;
        } else {
            println!("Block {} not found", block_number);
        }
    }

    Ok(())
}

// This function fetches and stores all transactions in a particular block
async fn sync_tx_data(block_number: u64) -> eyre::Result<()> {
    let provider = init_provider().await?;
    let supabase_client = SupabaseClient::new();

    let block_id = BlockId::Number(block_number.into());
    let transactions_kind = BlockTransactionsKind::Full;

    if let Some(block) = provider.get_block(block_id, transactions_kind).await? {
        if let BlockTransactions::Full(transactions) = block.transactions {
            for tx in &transactions {
                let tx_data = TransactionData {
                    block_number,
                    transaction_hash: format!("{:?}", tx.hash),
                    from: format!("{:?}", tx.from),
                    to: tx.to.map(|to| format!("{:?}", to)),
                    value: format!("{}", tx.value),
                    gas: tx.gas,
                    gas_price: tx.gas_price.map_or("None".to_string(), |gp| format!("{}", gp)),
                    input: format!("{:?}", tx.input),
                    nonce: tx.nonce,
                    transaction_index: tx.transaction_index.unwrap_or(0),
                    max_fee_per_gas: tx.max_fee_per_gas.map(|fee| format!("{}", fee)),
                    max_priority_fee_per_gas: tx.max_priority_fee_per_gas.map(|fee| format!("{}", fee)),
                    chain_id: tx.chain_id.map(|chain| format!("{:?}", chain)),
                };

                supabase_client.store_transaction_data(tx_data).await?;
            }

            println!("Synced {} transactions from block: {}", transactions.len(), block_number);
        } else {
            println!("No full transactions found in block {}", block_number);
        }
    } else {
        println!("Block {} not found", block_number);
    }

    Ok(())
}

