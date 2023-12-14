use ethers::prelude::abigen;
use ethers::providers::{Middleware, Provider};
use ethers::types::Address;
use std::sync::Arc;
const RPC_URL: &str = "https://api.avax.network/ext/bc/C/rpc";
use colorize::{self, AnsiColor};
use std::collections::HashMap;

abigen!(IJOEPair, "./abis/joe_lp_abi.json");
abigen!(IERC20, "./abis/erc20_abi.json");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider: Arc<Provider<ethers::providers::Http>> = Arc::new(Provider::try_from(RPC_URL)?);

    let chain_id = provider.get_chainid().await?;
    let block_number = provider.get_block_number().await?;

    println!("Chain ID: {}", chain_id);
    println!("Block Number: {}\n", block_number);

    let provider_clone = provider.clone();

    let task_0 = tokio::spawn(async move {
        get_reserves("WAVAX-USDT", provider).await
    });
    
    let task_1 = tokio::spawn(async move {
        get_reserves("WAVAX-USDC", provider_clone).await
    });

    for task in [task_0, task_1] {
        if let Ok((pair_name,reserve_0, reserve_1)) = task.await? {
            println!(
                "{}\nWAVAX Supply: {}\nUSDT Supply: {}\n",
                pair_name.green(),
                reserve_0,
                reserve_1
            );
        }
    }
    Ok(())
}


async fn get_reserves(pair_name: &str, provider:Arc<Provider<ethers::providers::Http>>) -> eyre::Result<(&str,u128,u128)> {

    let pair_address_map = get_pair_address_mapping();
    let pair_address = pair_address_map
        .get(pair_name)
        .ok_or_else(|| eyre::eyre!("Pair name not found"))?
        .to_owned();
    
    let pair = IJOEPair::new(pair_address, provider.clone());
    let token_x_address: Address = pair.get_token_x().call().await?;
    let token_y_address: Address = pair.get_token_y().call().await?;
    let x_token = IERC20::new(token_x_address, provider.clone());
    let y_token = IERC20::new(token_y_address, provider.clone());
    let x_decimal: u8 = x_token.decimals().call().await?;
    let y_decimal: u8 = y_token.decimals().call().await?;
    let (reserve_0, reserve_1) = pair.get_reserves().call().await?;
    Ok((pair_name, reserve_0 / 10u128.pow(x_decimal.into()), reserve_1 / 10u128.pow(y_decimal.into())))
}


fn get_pair_address_mapping() -> HashMap<&'static str, Address> {
    let mut map = HashMap::new();
    map.insert("WAVAX-USDT", "0x87EB2F90d7D0034571f343fb7429AE22C1Bd9F72".parse().unwrap());
    map.insert("WAVAX-USDC", "0xD446eb1660F766d533BeCeEf890Df7A69d26f7d1".parse().unwrap());
    map
}


