
use ethers::providers::{Provider, Middleware, Ws};
use ethers::core::types::transaction::response;

use std::sync::Arc;


pub async fn watch(http_provider:Arc<Provider<ethers::providers::Http>>) -> eyre::Result<()> {
    loop {
        let current_block_number = http_provider.get_block_number().await?;
        let mut last_block_number = None;
        match last_block_number {
            Some(last_number) if last_number == current_block_number => {
            },
            _ => {
                if let Some(full_block) = http_provider.get_block_with_txs(current_block_number).await? {
                    for transaction in full_block.transactions {
                        println!("Gas used: {:?}", transaction.to.unwrap());
                    }
                }
                last_block_number = Some(current_block_number);
            }
        }

        // Optional: add a delay to avoid querying too frequently
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    Ok(())
}
