use crate::binance::Binance;
use crate::bitstamp::Bitstamp;
use crate::exchange::Exchange;
use crate::order_book::OrderBook;
use core::cmp::Ordering;
pub use orderbook::{
    orderbook_aggregator_client::OrderbookAggregatorClient,
    orderbook_aggregator_server::OrderbookAggregatorServer, Empty, Level,
};
use orderbook::{orderbook_aggregator_server::OrderbookAggregator, Summary};
use std::collections::HashMap;
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
    exchanges: Arc<Mutex<HashMap<&'static str, OrderBook>>>,
}

impl OrderBookService {
    pub fn new(pair: &'static str) -> Self {
        Self {
            pair,
            exchanges: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn get_exchanges(&self) -> Arc<Mutex<HashMap<&'static str, OrderBook>>> {
        self.exchanges.clone()
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.price == other.price {
            return Some(self.amount.partial_cmp(&other.amount).unwrap());
        }
        Some(self.price.partial_cmp(&other.price).unwrap())
    }
}

async fn update_exchange(exchanges: &Arc<Mutex<HashMap<&str, OrderBook>>>, order_book: OrderBook) {
    let mut exchanges = exchanges.lock().await;
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
}

fn get_ten_first_levels(levels: Vec<Level>) -> Vec<Level> {
    levels.into_iter().take(10).collect()
}

async fn merge_levels(
    exchanges: &Arc<Mutex<HashMap<&'static str, OrderBook>>>,
) -> (Vec<Level>, Vec<Level>) {
    let (mut bids, mut asks) = exchanges.lock().await.values().fold(
        (vec![], vec![]),
        |(mut bids, mut asks), order_book| {
            bids.append(&mut order_book.get_bids());
            asks.append(&mut order_book.get_asks());
            (bids, asks)
        },
    );

    bids.sort_by(|a, b| b.partial_cmp(a).unwrap());
    asks.sort_by(|a, b| a.partial_cmp(b).unwrap());

    (get_ten_first_levels(bids), get_ten_first_levels(asks))
}

async fn get_summary(exchanges: &Arc<Mutex<HashMap<&'static str, OrderBook>>>) -> Summary {
    let (merged_bids, merged_asks) = merge_levels(exchanges).await;

    Summary {
        spread: merged_asks[0].price - merged_bids[0].price,
        bids: merged_bids,
        asks: merged_asks,
    }
}

#[tonic::async_trait]
impl OrderbookAggregator for OrderBookService {
    type BookSummaryStream = UnboundedReceiverStream<Result<Summary, Status>>;

    async fn book_summary(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::BookSummaryStream>, Status> {
        let exchanges: Arc<Mutex<HashMap<&str, OrderBook>>> = self.get_exchanges();

        let (order_book_tx, mut order_book_rx) = unbounded_channel::<OrderBook>();
        let (summary_tx, summary_rx) = unbounded_channel::<Result<Summary, Status>>();

        spawn(Bitstamp::get_order_book(self.pair, order_book_tx.clone()));
        spawn(Binance::get_order_book(self.pair, order_book_tx));
        spawn(async move {
            while let Some(order_book) = order_book_rx.recv().await {
                update_exchange(&exchanges, order_book).await;

                // TODO merge order books
                summary_tx.send(Ok(get_summary(&exchanges).await)).unwrap();
            }
        });

        Ok(Response::new(UnboundedReceiverStream::new(summary_rx)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::{get_binance_order_book_builder, get_bitstamp_order_book_builder};

    #[tokio::test]
    async fn test_get_summary() {
        let service = OrderBookService::new("ethbtc");
        let exchanges_mutex = service.get_exchanges();
        update_exchange(&exchanges_mutex, get_bitstamp_order_book_builder().build()).await;
        update_exchange(&exchanges_mutex, get_binance_order_book_builder().build()).await;

        let summary = get_summary(&exchanges_mutex).await;
        assert_eq!(summary.spread, 1.000000000001e-6);
        assert_eq!(summary.bids.len(), 10);
        assert_eq!(summary.asks.len(), 10);
        // test that bids with price 0.06842268 are sorted by amount
        assert!(summary.bids[3].amount > summary.bids[4].amount);
    }
}
