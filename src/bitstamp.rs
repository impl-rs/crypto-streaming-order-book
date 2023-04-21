use crate::exchange::Exchange;
use crate::order_book::{OrderBook, OrderBookBuilder};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::{connect_async, tungstenite::Message};

const BITSTAMP_WEB_SOCKET_URL: &str = "wss://ws.bitstamp.net/";

#[derive(Debug, Deserialize)]
pub struct Bitstamp;

#[tonic::async_trait]
impl Exchange for Bitstamp {
    fn get_name() -> &'static str {
        "bitstamp"
    }
    async fn get_order_book(pair: &str, sender: UnboundedSender<OrderBook>) -> () {
        let (ws_stream, _) = connect_async(BITSTAMP_WEB_SOCKET_URL).await.unwrap();
        let (mut write, read) = ws_stream.split();

        write
            .send(Message::Text(
                BitstampSubscription::new("bts:subscribe", format!("order_book_{}", pair))
                    .to_json(),
            ))
            .await
            .unwrap();

        read.for_each(|message| async {
            if let Ok(Message::Text(text)) = message {
                let response_event: BitstampResponseEvent = serde_json::from_str(&text).unwrap();
                if let BitstampWebSocketEvent::Data = response_event.event {
                    let bitstamp_response: BitstampResponse = serde_json::from_str(&text).unwrap();
                    let order_book: OrderBookBuilder<Bitstamp> = bitstamp_response.into();
                    sender.send(order_book.build()).unwrap();
                }
            }
        })
        .await;
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
            data: BitstampSubscriptionData { channel },
        }
    }
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Deserialize, Debug)]
struct BitstampResponse {
    data: OrderBookBuilder<Bitstamp>,
}

impl From<BitstampResponse> for OrderBookBuilder<Bitstamp> {
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
