use ethers::prelude::abigen;
use ethers::providers::{Middleware, Provider};
use ethers::types::Address;
use std::sync::Arc;
const RPC_URL: &str = "https://api.avax.network/ext/bc/C/rpc";

abigen!(IJOEPair, "./abis/joe_lp_abi.json");
abigen!(IERC20, "./abis/erc20_abi.json");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider: Arc<Provider<ethers::providers::Http>> = Arc::new(Provider::try_from(RPC_URL)?);

    let chain_id = provider.get_chainid().await?;
    let block_number = provider.get_block_number().await?;

    println!("Chain ID: {}", chain_id);
    println!("Block Number: {}", block_number);
    let wavax_usdt_pair_address: Address = "0x87EB2F90d7D0034571f343fb7429AE22C1Bd9F72".parse()?;
    let wavax_usdc_pair_address: Address = "0xD446eb1660F766d533BeCeEf890Df7A69d26f7d1".parse()?;

    let wavax_usdt_pair = IJOEPair::new(wavax_usdt_pair_address, provider.clone());
    let wavax_usdc_pair = IJOEPair::new(wavax_usdc_pair_address, provider.clone());

    // Use the get_reserves() function to fetch the pool reserves
    let wavax_token_address: Address = wavax_usdt_pair.get_token_x().call().await?;
    let usdt_token_address: Address = wavax_usdt_pair.get_token_y().call().await?;
    let wavax_token = IERC20::new(wavax_token_address, provider.clone());
    let usdt_token = IERC20::new(usdt_token_address, provider.clone());
    let wavax_decimal: u8 = wavax_token.decimals().call().await?;
    let usdt_decimal: u8 = usdt_token.decimals().call().await?;

    let (reserve_0, reserve_1) = wavax_usdt_pair.get_reserves().call().await?;
    println!(
        "WAVAX Supply: {}",
        reserve_0 / 10u128.pow(wavax_decimal.into())
    );
    println!(
        "USDT Supply: {}",
        reserve_1 / 10u128.pow(usdt_decimal.into())
    );
    let (reserve_0, reserve_1) = wavax_usdc_pair.get_reserves().call().await?;
    println!(
        "WAVAX Supply: {}",
        reserve_0 / 10u128.pow(wavax_decimal.into())
    );
    println!(
        "USDC Supply: {}",
        reserve_1 / 10u128.pow(usdt_decimal.into())
    );

    Ok(())
}


async fn get_reserves(pair_address: Address, provider:Arc<Provider<ethers::providers::Http>>) -> eyre::Result<(u128,u128)> {
    let pair = IJOEPair::new(pair_address, provider.clone());
    let token_x_address: Address = pair.get_token_x().call().await?;
    let token_y_address: Address = pair.get_token_y().call().await?;
    let x_token = IERC20::new(token_x_address, provider.clone());
    let y_token = IERC20::new(token_y_address, provider.clone());
    let x_decimal: u8 = x_token.decimals().call().await?;
    let y_decimal: u8 = y_token.decimals().call().await?;
    let (reserve_0, reserve_1) = pair.get_reserves().call().await?;
    Ok((reserve_0 / 10u128.pow(x_decimal.into()), reserve_1 / 10u128.pow(y_decimal.into())))
}

