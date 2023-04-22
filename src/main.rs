mod binance;
mod bitstamp;
mod exchange;
mod order_book;
mod proto;
mod service;
use crate::proto::OrderbookAggregatorServer;
use crate::service::OrderBookService;
use anyhow::Result;
use std::error::Error;
use tonic::transport::Server;
#[cfg(test)]
mod test_data;

#[cfg(test)]
mod test_server;

async fn start_server(pair: &'static str) -> Result<()> {
    let addresse = "[::1]:10000".parse().unwrap();

    println!("OrderbookAggregatorServer listening on: {}", addresse);

    let order_book_service = OrderBookService::new(pair);

    let order_book_server = OrderbookAggregatorServer::new(order_book_service);

    Server::builder()
        .add_service(order_book_server)
        .serve(addresse)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // server
    let pair = "ethbtc";
    start_server(pair).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::{Empty, OrderbookAggregatorClient, Summary};
    use crate::test_data::{get_binance_websocket_response, get_bitstamp_websocket_response};
    use crate::test_server::TestServer;
    use anyhow::Result;
    use tokio::spawn;
    use tokio::time::{sleep, Duration};
    use tonic::Streaming;

    async fn start_client() -> Result<Streaming<Summary>> {
        let mut client = OrderbookAggregatorClient::connect("http://[::1]:10000").await?;

        let stream = client.book_summary(Empty {}).await?.into_inner();

        Ok(stream)
    }

    #[tokio::test]
    async fn test_service() -> Result<()> {
        let mut binance_server = TestServer::new("8080").await;
        let mut bitstamp_server = TestServer::new("8081").await;

        spawn(start_server("ethbtc"));
        // wait for server to start (kinda hacky but works for now)
        sleep(Duration::from_millis(1000)).await;

        let mut stream = start_client().await?;

        // First we send a message from binance
        binance_server.send_message(get_binance_websocket_response());

        // Then we wait for a summary from the stream and assert the data
        if let Some(summary) = stream.message().await? {
            let Summary { bids, asks, .. } = summary;
            assert_eq!(bids.len(), 10);
            assert_eq!(asks.len(), 10);

            // highest price first for bids
            assert_eq!(bids[0].price, 0.067955);
            assert_eq!(bids[9].price, 0.067945);

            // lowest price first for asks
            assert_eq!(asks[0].price, 0.067956);
            assert_eq!(asks[9].price, 0.067966);
        }

        // Then we send a message from bitstamp
        bitstamp_server.send_message(get_bitstamp_websocket_response());

        // Then we wait for a summary from the stream and assert the data
        if let Some(summary) = stream.message().await? {
            let Summary { bids, asks, .. } = summary;
            assert_eq!(bids.len(), 10);
            assert_eq!(asks.len(), 10);

            // highest price first for bids
            assert_eq!(bids[0].price, 0.067955);
            assert_eq!(bids[9].price, 0.067945);

            // asks are different after bitstamp data
            assert_eq!(asks[0].price, 0.06792853);
            assert_eq!(asks[9].price, 0.06795205);
        }

        Ok(())
    }
}
