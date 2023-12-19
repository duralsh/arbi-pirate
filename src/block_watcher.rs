
use ethers::providers::{Provider, Middleware, Ws};
use ethers::providers::StreamExt;
const RPC_URL: &str = "wss://api.avax.network/ext/bc/C/ws";


pub async fn watch() -> eyre::Result<()> {
    let provider = Provider::<Ws>::connect(RPC_URL).await?;
    let mut stream = provider.subscribe_full_pending_txs().await?;
    while let Some(block) = stream.next().await {
        println!(
            "\n\nTX : {:?}\n\n",
            block
        );
    }
    Ok(())
}
