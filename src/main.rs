use ethers::prelude::abigen;
use ethers::providers::{Middleware, Provider};
use ethers::types::Address;
use std::sync::Arc;
const RPC_URL: &str = "https://api.avax.network/ext/bc/C/rpc";

abigen!(IJOEPair, "./abis/joe_lp_abi.json");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = Arc::new(Provider::try_from(RPC_URL)?);

    let chain_id = provider.get_chainid().await?;
    let block_number = provider.get_block_number().await?;

    println!("Chain ID: {}", chain_id);
    println!("Block Number: {}", block_number);
    let wavax_usdt_pair_address: Address = "0x87EB2F90d7D0034571f343fb7429AE22C1Bd9F72".parse()?;
    let wavax_usdt_pair = IJOEPair::new(wavax_usdt_pair_address, provider);

    // Use the get_reserves() function to fetch the pool reserves
    let (reserve_0, reserve_1) = wavax_usdt_pair.get_reserves().call().await?;
    println!("Reserve 0: {}", reserve_0);
    println!("Reserve 1: {}", reserve_1);

    Ok(())
}
