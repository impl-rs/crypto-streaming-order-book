use crate::exchange::Exchange;
use crate::order_book::Level;
use anyhow::Result;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::Message};
const BINANCE_WEB_SOCKET_URL: &str = "wss://stream.binance.com:9443/ws/";

#[derive(Debug)]
pub struct Binance {}

impl Exchange for Binance {
    fn get_name() -> &'static str {
        "binance"
    }
}

impl Binance {
    pub async fn get_order_book() -> Result<()> {
        let subscription = BinanceSubscription::new("btcbusd", 10, 100);
        let (ws_stream, _) = connect_async(subscription.to_url()).await?;
        let (_, read) = ws_stream.split();

        read.for_each(|message| async {
            if let Ok(Message::Text(text)) = message {
                let response: BinanceResponseData = serde_json::from_str(&text).unwrap();
                println!("binance response: {:#?}", response.bids[0]);
            }
        })
        .await;

        Ok(())
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

#[derive(Deserialize, Debug)]
struct BinanceResponseData {
    lastUpdateId: i64,
    bids: Vec<Level<Binance>>,
    asks: Vec<Level<Binance>>,
}
