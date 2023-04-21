pub use orderbook::{
    orderbook_aggregator_client::OrderbookAggregatorClient,
    orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer},
    Empty, Level, Summary,
};

mod orderbook {
    tonic::include_proto!("orderbook");
}
