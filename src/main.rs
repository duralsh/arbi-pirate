use ethers::prelude::abigen;
use ethers::providers::{Middleware, Provider};
use ethers::types::Address;
use std::sync::Arc;
const RPC_URL: &str = "https://api.avax.network/ext/bc/C/rpc";
use colorize::{self, AnsiColor};

abigen!(IJOEPair, "./abis/joe_lp_abi.json");
abigen!(IERC20, "./abis/erc20_abi.json");

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider: Arc<Provider<ethers::providers::Http>> = Arc::new(Provider::try_from(RPC_URL)?);

    let chain_id = provider.get_chainid().await?;
    let block_number = provider.get_block_number().await?;

    println!("Chain ID: {}", chain_id);
    println!("Block Number: {}\n", block_number);
    let wavax_usdt_pair_address: Address = "0x87EB2F90d7D0034571f343fb7429AE22C1Bd9F72".parse()?;
    let wavax_usdc_pair_address: Address = "0xD446eb1660F766d533BeCeEf890Df7A69d26f7d1".parse()?;
    let (reserve_0, reserve_1) = get_reserves(wavax_usdt_pair_address, provider.clone()).await?;
    println!(
        "{}\nWAVAX Supply: {}\nUSDT Supply: {}",
        "WAVAX-USDT JOE LP".green(),
        reserve_0,
        reserve_1
    );
    let (reserve_0, reserve_1) = get_reserves(wavax_usdc_pair_address, provider.clone()).await?;
    println!(
        "{}\nWAVAX Supply: {}\nUSDC Supply: {}",
        "WAVAX-USDT JOE LP".green(),
        reserve_0,
        reserve_1
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

