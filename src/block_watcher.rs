
use ethers::providers::{Provider, Middleware};
#[allow(unused_imports)]
use ethers::core::types::transaction::response;

use std::sync::Arc;


pub async fn watch(http_provider:Arc<Provider<ethers::providers::Http>>) -> eyre::Result<()> {

    let mut last_block_number = None;

    loop {
        let current_block_number = http_provider.get_block_number().await?;
        match last_block_number {
            Some(last_number) if last_number == current_block_number || current_block_number < last_number => {
            },
            _ => {
                if let Some(full_block) = http_provider.get_block_with_txs(current_block_number).await? {
                    for transaction in full_block.transactions {
                        match transaction.to {
                            Some(to_address) => println!("Block No: {} To Address: {:?}", current_block_number, to_address),
                            None => println!("Block No: {} To Address: None", current_block_number),
                        }
                    }
                }
                last_block_number = Some(current_block_number);
            }
        }
    }

}
