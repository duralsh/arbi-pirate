
use ethers::providers::{Provider, Middleware};
use ethers::core::types::transaction::response;

use std::sync::Arc;


pub async fn watch(http_provider:Arc<Provider<ethers::providers::Http>>) -> eyre::Result<()> {

    let mut last_block_number = None;

    loop {
        let current_block_number = http_provider.get_block_number().await?;
        println!("Current Block Number: {}", current_block_number);
        match last_block_number {
            Some(last_number) if last_number == current_block_number => {
            },
            _ => {
                if let Some(full_block) = http_provider.get_block_with_txs(current_block_number).await? {
                    for transaction in full_block.transactions {
                        // println!("Block No :{} To Address: {:?}", current_block_number, transaction.to.unwrap());
                    }
                }
                last_block_number = Some(current_block_number);
            }
        }

        // Optional: add a delay to avoid querying too frequently
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }

}
