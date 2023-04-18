mod binance;
mod bitstamp;
mod exchange;
mod order_book;
mod service;
use crate::service::{Empty, OrderbookAggregatorClient};

#[cfg(test)]
mod test_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = OrderbookAggregatorClient::connect("http://[::1]:10000").await?;

    let mut stream = client.book_summary(Empty {}).await?.into_inner();

    while let Some(summary) = stream.message().await? {
        println!("summary = {:?}", summary);
    }

    Ok(())
}
