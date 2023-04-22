mod binance;
mod bitstamp;
mod exchange;
mod order_book;
mod proto;
mod service;
use crate::proto::OrderbookAggregatorServer;
use crate::service::OrderBookService;
use std::error::Error;
use tonic::transport::Server;
#[cfg(test)]
mod test_data;

#[cfg(test)]
mod test_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // server
    let pair = "ethbtc";
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
