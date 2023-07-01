use crate::binance::Binance;
use crate::bitstamp::Bitstamp;
use crate::exchange::Exchange;
use crate::order_book::OrderBook;
use crate::proto::{Empty, Level, OrderbookAggregator, Summary};
use core::cmp::Ordering;
use futures_util::ready;
use futures_util::task::Context;
use futures_util::task::Poll;
use futures_util::Stream;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::broadcast::error::RecvError;
use tokio::{
    spawn,
    sync::{broadcast, mpsc, Mutex},
};
use tokio_util::sync::ReusableBoxFuture;

use tonic::{Request, Response, Status};
const CHANNEL_BUFFER_SIZE: usize = 100;

#[derive(Clone)]
pub struct Connected;

#[derive(Clone)]
pub struct NotConnected;

pub struct OrderBookService<ServiceStatus = NotConnected> {
    pair: String,
    exchanges: Arc<Mutex<HashMap<&'static str, OrderBook>>>,
    status: PhantomData<ServiceStatus>,
    summary_sender: Option<broadcast::Sender<Summary>>,
}

impl OrderBookService {
    pub fn new(pair: String) -> Self {
        Self {
            pair,
            exchanges: Arc::new(Mutex::new(HashMap::new())),
            status: PhantomData,
            summary_sender: None,
        }
    }
    pub fn connect_exchanges(self) -> OrderBookService<Connected> {
        let exchanges: Arc<Mutex<HashMap<&str, OrderBook>>> = self.get_exchanges();

        let (order_book_tx, mut order_book_rx) = mpsc::channel::<OrderBook>(CHANNEL_BUFFER_SIZE);
        let (summary_tx, _summary_rx) = broadcast::channel::<Summary>(CHANNEL_BUFFER_SIZE);

        spawn(Bitstamp::get_order_book(
            self.pair.clone(),
            order_book_tx.clone(),
        ));
        spawn(Binance::get_order_book(self.pair.clone(), order_book_tx));

        let exchanges_clone = exchanges.clone();
        let summary_tx_clone = summary_tx.clone();

        spawn(async move {
            while let Some(order_book) = order_book_rx.recv().await {
                update_exchange(&exchanges_clone, order_book).await;

                if summary_tx_clone
                    .send(get_summary(&exchanges_clone).await)
                    .is_ok()
                {
                    println!("Summary sent")
                }
            }
        });

        OrderBookService {
            pair: self.pair,
            exchanges,
            status: PhantomData,
            summary_sender: Some(summary_tx),
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
        |(mut acc_bids, mut acc_asks), order_book| {
            let (mut bids, mut asks) = order_book.get_levels();
            acc_bids.append(&mut bids);
            acc_asks.append(&mut asks);
            (acc_bids, acc_asks)
        },
    );

    bids.sort_by(|a, b| b.partial_cmp(a).unwrap());
    asks.sort_by(|a, b| a.partial_cmp(b).unwrap());

    (get_ten_first_levels(bids), get_ten_first_levels(asks))
}

async fn get_summary(exchanges: &Arc<Mutex<HashMap<&'static str, OrderBook>>>) -> Summary {
    let (merged_bids, merged_asks) = merge_levels(exchanges).await;

    let spread = match (merged_asks.first(), merged_bids.first()) {
        (Some(first_ask), Some(first_bid)) => first_ask.price - first_bid.price,
        _ => f64::NAN,
    };

    Summary {
        spread,
        bids: merged_bids,
        asks: merged_asks,
    }
}

pub struct OrderBookSummaryStream {
    inner: ReusableBoxFuture<'static, (Result<Summary, RecvError>, broadcast::Receiver<Summary>)>,
}

async fn make_future(
    mut rx: broadcast::Receiver<Summary>,
) -> (Result<Summary, RecvError>, broadcast::Receiver<Summary>) {
    let result = rx.recv().await;
    (result, rx)
}

impl OrderBookSummaryStream {
    pub fn new(summary_rx: broadcast::Receiver<Summary>) -> Self {
        Self {
            inner: ReusableBoxFuture::new(make_future(summary_rx)),
        }
    }
}

impl Stream for OrderBookSummaryStream {
    type Item = Result<Summary, Status>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let (result, rx) = ready!(self.inner.poll(cx));
        self.inner.set(make_future(rx));
        match result {
            Ok(item) => Poll::Ready(Some(Ok(item))),
            Err(RecvError::Closed) => Poll::Ready(None),
            Err(RecvError::Lagged(_)) => Poll::Ready(Some(Err(Status::internal("Message lagged")))),
        }
    }
}

#[tonic::async_trait]
impl OrderbookAggregator for OrderBookService<Connected> {
    type BookSummaryStream = OrderBookSummaryStream;

    async fn book_summary(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::BookSummaryStream>, Status> {
        let OrderBookService { summary_sender, .. } = self;
        if let Some(sender) = summary_sender {
            return Ok(Response::new(OrderBookSummaryStream::new(
                sender.subscribe(),
            )));
        }
        Err(Status::internal("Summary stream not initialized"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::{get_binance_order_book_builder, get_bitstamp_order_book_builder};

    #[tokio::test]
    async fn test_get_summary() {
        let service = OrderBookService::new("ethbtc".into());
        let exchanges_mutex = service.get_exchanges();
        update_exchange(&exchanges_mutex, get_bitstamp_order_book_builder().build()).await;
        update_exchange(&exchanges_mutex, get_binance_order_book_builder().build()).await;

        let summary = get_summary(&exchanges_mutex).await;
        assert_eq!(summary.spread, 1.000000000001e-6);

        assert_eq!(summary.bids.len(), 10);
        assert_eq!(summary.asks.len(), 10);

        // highest price first for bids
        assert_eq!(summary.bids[0].price, 0.068426);
        assert_eq!(summary.bids[9].price, 0.06842);

        // lowest price first for asks
        assert_eq!(summary.asks[0].price, 0.068427);
        assert_eq!(summary.asks[9].price, 0.068437);

        // test that bids with price 0.06842268 are sorted by amount
        assert!(summary.bids[3].price == summary.bids[4].price);
        assert!(summary.bids[3].amount > summary.bids[4].amount);
    }
}
