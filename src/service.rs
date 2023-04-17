use crate::binance::Binance;
use crate::bitstamp::Bitstamp;
use crate::order_book::OrderBook;
pub use orderbook::{
    orderbook_aggregator_client::OrderbookAggregatorClient,
    orderbook_aggregator_server::OrderbookAggregatorServer, Empty,
};
use orderbook::{orderbook_aggregator_server::OrderbookAggregator, Summary};
use tokio::{spawn, sync::mpsc::unbounded_channel};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

#[derive(Debug)]
pub struct OrderBookService {
    pub pair: &'static str,
    // TODO add state from order book before merging
}

#[tonic::async_trait]
impl OrderbookAggregator for OrderBookService {
    type BookSummaryStream = UnboundedReceiverStream<Result<Summary, Status>>;

    async fn book_summary(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::BookSummaryStream>, Status> {
        dbg!("here");
        let (order_book_tx, mut order_book_rx) = unbounded_channel::<OrderBook>();
        let (summary_tx, summary_rx) = unbounded_channel::<Result<Summary, Status>>();

        spawn(Bitstamp::get_order_book(self.pair, order_book_tx.clone()));
        spawn(Binance::get_order_book(self.pair, order_book_tx));
        spawn(async move {
            while let Some(order_book) = order_book_rx.recv().await {
                dbg!(order_book);
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
