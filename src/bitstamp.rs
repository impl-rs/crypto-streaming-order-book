use crate::exchange::Exchange;
use crate::order_book::{OrderBook, OrderBookBuilder};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[cfg(not(test))]
const BITSTAMP_WEB_SOCKET_URL: &str = "wss://ws.bitstamp.net/";

#[cfg(test)]
const BITSTAMP_WEB_SOCKET_URL: &str = "ws://localhost:8081/ws/";

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::get_bitstamp_websocket_response;
    use crate::test_server::TestServer;
    use tokio::spawn;
    #[tokio::test]
    async fn test_bitstamp_websocket() {
        let mut server = TestServer::new("8081").await;
        let (sender, mut receiver) = server.get_channels();

        spawn(Bitstamp::get_order_book("ethbtc", sender));

        server.send_message(get_bitstamp_websocket_response());

        if let Some(order_book) = receiver.recv().await {
            assert_eq!(order_book.get_exchange_name(), "bitstamp");
            let (bids, asks) = order_book.get_levels();

            assert_eq!(bids.len(), 10);
            assert_eq!(asks.len(), 10);

            // highest price first for bids
            assert_eq!(bids[0].price, 0.06791795);
            assert_eq!(bids[9].price, 0.06789064);

            // lowest price first for asks
            assert_eq!(asks[0].price, 0.06792853);
            assert_eq!(asks[9].price, 0.06795205);
        }
    }
}
