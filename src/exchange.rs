use crate::order_book::OrderBook;
use tokio::sync::mpsc::Sender;
#[tonic::async_trait]
pub trait Exchange {
    fn get_name() -> &'static str;
    async fn get_order_book(pair: String, sender: Sender<OrderBook>) -> ();
}
