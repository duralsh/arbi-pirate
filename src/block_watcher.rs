
use ethers::providers::{Provider, Middleware};
#[allow(unused_imports)]
use ethers::core::types::transaction::response;
use ethers::types::Bytes;
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
                      
                        match decode_bytes_to_string(&transaction.input) {
                            Ok(decoded_string) => println!("Decoded string: {}", decoded_string),
                            Err(e) => eprintln!("Failed to decode Bytes to string: {}", e),
                        }
                    }
                }
                last_block_number = Some(current_block_number);
            }
        }
    }

}

fn decode_bytes_to_string(bytes: &Bytes) -> Result<String, std::string::FromUtf8Error> {
    // Convert the Bytes to a byte slice
    let byte_slice = bytes.as_ref();

    // Convert the byte slice to a UTF-8 string
    String::from_utf8(byte_slice.to_vec())
}
