use crate::order_book::OrderBook;
use tokio::sync::mpsc::UnboundedSender;
#[tonic::async_trait]
pub trait Exchange {
    fn get_name() -> &'static str;
    async fn get_order_book(pair: &str, sender: UnboundedSender<OrderBook>) -> ();
}
