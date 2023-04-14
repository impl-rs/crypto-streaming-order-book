use crate::exchange::Exchange;
use crate::order_book::OrderBook;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};
const BITSTAMP_WEB_SOCKET_URL: &str = "wss://ws.bitstamp.net/";

#[derive(Debug, Deserialize)]
pub struct Bitstamp {}

impl Exchange for Bitstamp {
    fn get_name() -> &'static str {
        "bitstamp"
    }
}

impl Bitstamp {
    pub async fn get_order_book(pair: &str) -> Result<()> {
        let (ws_stream, _) = connect_async(BITSTAMP_WEB_SOCKET_URL).await?;
        let (mut write, read) = ws_stream.split();

        write
            .send(Message::Text(
                BitstampSubscription::new("bts:subscribe", format!("order_book_{}", pair))
                    .to_json(),
            ))
            .await?;

        read.for_each(|message| async {
            if let Ok(Message::Text(text)) = message {
                let response_event: BitstampResponseEvent = serde_json::from_str(&text).unwrap();
                if let BitstampWebSocketEvent::Data = response_event.event {
                    let bitstamp_response: BitstampResponse = serde_json::from_str(&text).unwrap();
                    let order_book: OrderBook<Bitstamp> = bitstamp_response.into();
                    println!("bitstamp response: {:#?}", order_book.bids[0]);
                }
            }
        })
        .await;

        Ok(())
    }
}

// Channel subscriptions
#[derive(Serialize)]
struct BitstampSubscriptionData {
    channel: String,
}

#[derive(Serialize)]
struct BitstampSubscription {
    event: String,
    data: BitstampSubscriptionData,
}

impl BitstampSubscription {
    fn new(event: &str, channel: String) -> Self {
        Self {
            event: event.to_string(),
            data: BitstampSubscriptionData { channel: channel },
        }
    }
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Deserialize, Debug)]
struct BitstampResponse {
    data: OrderBook<Bitstamp>,
}

impl From<BitstampResponse> for OrderBook<Bitstamp> {
    fn from(bitstamp_response: BitstampResponse) -> Self {
        bitstamp_response.data
    }
}

#[derive(Deserialize, Debug)]
enum BitstampWebSocketEvent {
    #[serde(rename = "bts:subscription_succeeded")]
    BtsSubscriptionSucceded,
    #[serde(rename = "data")]
    Data,
}
#[derive(Deserialize, Debug)]
struct BitstampResponseEvent {
    event: BitstampWebSocketEvent,
}
