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
