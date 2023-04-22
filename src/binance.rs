use crate::exchange::Exchange;
use crate::order_book::{OrderBook, OrderBookBuilder};
use futures_util::StreamExt;
use serde::Deserialize;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[cfg(not(test))]
const BINANCE_WEB_SOCKET_URL: &str = "wss://stream.binance.com:9443/ws/";

#[cfg(test)]
const BINANCE_WEB_SOCKET_URL: &str = "ws://localhost:8080/ws/";

#[derive(Debug, Deserialize)]
pub struct Binance;

#[tonic::async_trait]
impl Exchange for Binance {
    fn get_name() -> &'static str {
        "binance"
    }
    async fn get_order_book(pair: &str, sender: UnboundedSender<OrderBook>) -> () {
        let subscription = BinanceSubscription::new(pair, 10, 100);
        let (ws_stream, _) = connect_async(subscription.to_url()).await.unwrap();
        let (_, read) = ws_stream.split();

        read.for_each(|message| async {
            if let Ok(Message::Text(text)) = message {
                let response: OrderBookBuilder<Binance> = serde_json::from_str(&text).unwrap();
                sender.send(response.build()).unwrap();
            }
        })
        .await;
    }
}

struct BinanceSubscription {
    pair: String,
    depth: i32,
    update_speed: i32,
}

impl BinanceSubscription {
    fn new(pair: &str, depth: i32, update_speed: i32) -> Self {
        Self {
            pair: pair.into(),
            depth,
            update_speed,
        }
    }
    fn to_url(&self) -> String {
        format!(
            "{}{}@depth{}@{}ms",
            BINANCE_WEB_SOCKET_URL, self.pair, self.depth, self.update_speed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_server::TestServer;
    use tokio::spawn;
    #[tokio::test]
    async fn test_binance_websocket() {
        let mut server = TestServer::new("8080").await;
        let (sender, mut receiver) = server.get_channels();

        spawn(Binance::get_order_book("ethbtc", sender));

        server.send_message("{\"lastUpdateId\":6630723519,\"bids\":[[\"0.06795500\",\"20.71540000\"],[\"0.06795400\",\"0.20000000\"],[\"0.06795300\",\"1.30030000\"],[\"0.06795100\",\"1.53340000\"],[\"0.06795000\",\"2.10520000\"],[\"0.06794900\",\"0.69620000\"],[\"0.06794800\",\"0.64150000\"],[\"0.06794700\",\"0.18030000\"],[\"0.06794600\",\"1.09700000\"],[\"0.06794500\",\"4.49010000\"]],\"asks\":[[\"0.06795600\",\"13.99720000\"],[\"0.06795700\",\"3.72720000\"],[\"0.06795800\",\"5.28970000\"],[\"0.06795900\",\"0.02970000\"],[\"0.06796000\",\"1.76030000\"],[\"0.06796100\",\"0.04260000\"],[\"0.06796300\",\"0.06960000\"],[\"0.06796400\",\"0.60260000\"],[\"0.06796500\",\"1.00340000\"],[\"0.06796600\",\"0.06620000\"]]}");

        if let Some(order_book) = receiver.recv().await {
            assert_eq!(order_book.get_exchange_name(), "binance");
            let (bids, asks) = order_book.get_levels();

            assert_eq!(bids.len(), 10);
            assert_eq!(asks.len(), 10);

            // highest price first for bids
            assert_eq!(bids[0].price, 0.067955);
            assert_eq!(bids[9].price, 0.067945);

            // lowest price first for asks
            assert_eq!(asks[0].price, 0.067956);
            assert_eq!(asks[9].price, 0.067966);
        }
    }
}
