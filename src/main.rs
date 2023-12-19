use ethers::prelude::abigen;
use ethers::providers::{Middleware, Provider};
use ethers::types::Address;
use pair::Pair;
use std::sync::Arc;
const RPC_URL: &str = "https://api.avax.network/ext/bc/C/rpc";
use std::collections::HashMap;
use ethers::types::U256;
use block_watcher::watch;

mod pair;
mod block_watcher;
abigen!(IJOEPair, "./abis/joe_lp_abi.json");
abigen!(IERC20, "./abis/erc20_abi.json");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider: Arc<Provider<ethers::providers::Http>> = Arc::new(Provider::try_from(RPC_URL)?);

    let watch_task = tokio::spawn(watch(provider.clone()));

    let chain_id = provider.get_chainid().await?;
    let block_number = provider.get_block_number().await?;

    println!("Chain ID: {}", chain_id);
    println!("Block Number: {}\n", block_number);

   let tasks = vec![
        tokio::spawn(get_reserves("WAVAX-USDT", provider.clone())),
        tokio::spawn(get_reserves("WAVAX-USDC", provider.clone())),
        tokio::spawn(get_reserves("USDT-USDC", provider.clone())),
    ];

    for task in tasks {
        if let Ok(pair) = task.await? {
            println!("{}", pair);
        }
    }
    watch_task.await??;
    Ok(())
}


async fn get_reserves(pair_name: &str, provider:Arc<Provider<ethers::providers::Http>>) -> eyre::Result<Pair> {

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

    let active_bin:u32 = pair.get_active_id().call().await?;
    let raw_price: U256 = pair.get_price_from_id(active_bin).call().await?;
    let price = adjust_price(raw_price, x_decimal, y_decimal);
    Ok(Pair::new(pair_name, reserve_0 / 10u128.pow(x_decimal.into()), reserve_1 / 10u128.pow(y_decimal.into()), price))
}


fn get_pair_address_mapping() -> HashMap<&'static str, Address> {
    let mut map = HashMap::new();
    map.insert("WAVAX-USDT", "0x87EB2F90d7D0034571f343fb7429AE22C1Bd9F72".parse().unwrap());
    map.insert("WAVAX-USDC", "0xD446eb1660F766d533BeCeEf890Df7A69d26f7d1".parse().unwrap());
    map.insert("USDT-USDC", "0x9B2Cc8E6a2Bbb56d6bE4682891a91B0e48633c72".parse().unwrap());
    map
}


fn adjust_price(price: U256, decimals_x: u8, decimals_y: u8) -> f64 {
    
    
    let price_f64 =    convert_fixed_point(price);
    let multiplier = 10f64.powi(decimals_x as i32 - decimals_y as i32);

    price_f64 * multiplier
}

fn convert_fixed_point(value: U256) -> f64 {
    let integer_part = value >> 128; // Shift right by 128 bits to get the integer part
    let fractional_part = value - (integer_part << 128); // Subtract the integer part shifted back to get the fractional part

    // Convert both parts to f64. Be cautious about precision loss.
    let integer_f64 = integer_part.as_u128() as f64;
    let fractional_f64 = fractional_part.as_u128() as f64 * 2f64.powi(-128);
    integer_f64 + fractional_f64
}