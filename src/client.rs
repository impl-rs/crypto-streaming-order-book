mod proto;
use crate::proto::{Empty, OrderbookAggregatorClient};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = OrderbookAggregatorClient::connect("http://[::1]:10000").await?;

    let mut stream = client.book_summary(Empty {}).await?.into_inner();

    while let Some(summary) = stream.message().await? {
        println!("summary = {:?}", summary);
    }

    Ok(())
}
