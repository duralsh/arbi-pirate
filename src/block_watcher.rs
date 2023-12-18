
use ethers::providers::{Provider, Middleware, Ws};
use ethers::providers::StreamExt;
const RPC_URL: &str = "https://api.avax.network/ext/bc/C/rpc";


pub async fn watch() -> eyre::Result<()> {
    let provider = Provider::<Ws>::connect(RPC_URL).await?;
    let mut stream = provider.subscribe_blocks().await?.take(1);
    println!("Listening for new blocks");
    while let Some(block) = stream.next().await {
        println!(
            "Ts: {:?}, block number: {} -> {:?}",
            block.timestamp,
            block.number.unwrap(),
            block.hash.unwrap()
        );
    }
    Ok(())
}
