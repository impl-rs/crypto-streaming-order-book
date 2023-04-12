use crate::exchange::Exchange;
use crate::order_book::{deserialize_bid_ask, BidAsk};
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};
const BITSTAMP_WEB_SOCKET_URL: &str = "wss://ws.bitstamp.net/";

pub struct Bitstamp {}

impl Exchange for Bitstamp {
    fn get_name() -> String {
        "bitstamp".to_string()
    }
}

impl Bitstamp {
    pub async fn get_order_book() -> Result<()> {
        let (ws_stream, _) = connect_async(BITSTAMP_WEB_SOCKET_URL).await?;
        let (mut write, read) = ws_stream.split();

        write
            .send(Message::Text(
                BitstampSubscription::new("bts:subscribe", "order_book_btcusd").to_json(),
            ))
            .await?;

        read.for_each(|message| async {
            if let Ok(Message::Text(text)) = message {
                let response_event: BitstampResponseEvent = serde_json::from_str(&text).unwrap();
                if let BitstampWebSocketEvent::Data = response_event.event {
                    let response: BitstampResponse = serde_json::from_str(&text).unwrap();
                    println!("bitstamp response: {:#?}", response.data.bids[0]);
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
    fn new(event: &str, channel: &str) -> Self {
        Self {
            event: event.to_string(),
            data: BitstampSubscriptionData {
                channel: channel.to_string(),
            },
        }
    }
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Deserialize, Debug)]
struct BitstampResponseData {
    timestamp: String,
    microtimestamp: String,
    #[serde(deserialize_with = "deserialize_bid_ask::<'_, _, Bitstamp>")]
    bids: Vec<BidAsk>,
    #[serde(deserialize_with = "deserialize_bid_ask::<'_, _, Bitstamp>")]
    asks: Vec<BidAsk>,
}

#[derive(Deserialize, Debug)]
struct BitstampResponse {
    data: BitstampResponseData,
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
