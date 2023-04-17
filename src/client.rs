mod binance;
mod bitstamp;
mod exchange;
mod order_book;
mod service;
use crate::service::{Empty, OrderBookService, OrderbookAggregatorClient};
use futures_util::StreamExt;
use tonic::transport::Channel;

#[cfg(test)]
mod test_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = OrderbookAggregatorClient::connect("http://[::1]:10000").await?;
    dbg!("client connected");
    let mut stream = client.book_summary(Empty {}).await?.into_inner();
    dbg!("stream created, listening....");
    while let Some(summary) = stream.message().await? {
        println!("summary = {:?}", summary);
    }

    Ok(())
}
