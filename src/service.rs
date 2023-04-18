use std::collections::HashMap;

use crate::binance::Binance;
use crate::bitstamp::Bitstamp;
use crate::order_book::OrderBook;
pub use orderbook::{
    orderbook_aggregator_client::OrderbookAggregatorClient,
    orderbook_aggregator_server::OrderbookAggregatorServer, Empty,
};
use orderbook::{orderbook_aggregator_server::OrderbookAggregator, Summary};
use std::sync::Arc;
use tokio::{
    spawn,
    sync::{mpsc::unbounded_channel, Mutex},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

#[derive(Debug)]
pub struct OrderBookService {
    pair: &'static str,
    // TODO add state from order book before merging
    pub exchanges: Arc<Mutex<HashMap<&'static str, OrderBook>>>,
}

impl OrderBookService {
    pub fn new(pair: &'static str) -> Self {
        Self {
            pair,
            exchanges: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn get_exchanges(&self) -> Arc<Mutex<HashMap<&'static str, OrderBook>>> {
        self.exchanges.clone()
    }
}

#[tonic::async_trait]
impl OrderbookAggregator for OrderBookService {
    type BookSummaryStream = UnboundedReceiverStream<Result<Summary, Status>>;

    async fn book_summary(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::BookSummaryStream>, Status> {
        let exhanges = self.get_exchanges();

        let (order_book_tx, mut order_book_rx) = unbounded_channel::<OrderBook>();
        let (summary_tx, summary_rx) = unbounded_channel::<Result<Summary, Status>>();

        spawn(Bitstamp::get_order_book(self.pair, order_book_tx.clone()));
        spawn(Binance::get_order_book(self.pair, order_book_tx));
        spawn(async move {
            while let Some(order_book) = order_book_rx.recv().await {
                let mut exchanges = exhanges.lock().await;
                let order_book_exchange_name = order_book.get_exchange_name();
                // if key exists, update value to avoid cloning
                if exchanges.contains_key(order_book_exchange_name) {
                    exchanges
                        .entry(order_book_exchange_name)
                        .and_modify(|e| *e = order_book);
                } else {
                    exchanges
                        .entry(order_book_exchange_name)
                        .or_insert(order_book);
                }

                // TODO merge order books
                summary_tx
                    .send(Ok(Summary {
                        spread: 0.0,
                        bids: vec![],
                        asks: vec![],
                    }))
                    .unwrap();
            }
        });

        Ok(Response::new(UnboundedReceiverStream::new(summary_rx)))
    }
}
