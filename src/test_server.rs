use crate::order_book::OrderBook;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_tungstenite::{accept_async, tungstenite::Result};

async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
    websocket_rx: &mut Receiver<String>,
) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    println!("New WebSocket connection: {}", peer);
    let (mut ws_sender, _ws_receiver) = ws_stream.split();

    while let Some(msg) = websocket_rx.recv().await {
        ws_sender.send(msg.into()).await?;
    }

    Ok(())
}

pub struct TestServer {
    pub websocket_tx: Sender<String>,
}

impl TestServer {
    pub async fn new(port: &str) -> Self {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await.expect("Can't listen");
        let (websocket_tx, mut websocket_rx) = channel::<String>(100);

        println!("Listening on: {}", addr);

        spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let peer = stream
                    .peer_addr()
                    .expect("connected streams should have a peer address");
                println!("Peer address: {}", peer);

                // can only accept one connection at a time
                handle_connection(peer, stream, &mut websocket_rx)
                    .await
                    .unwrap();
            }
        });

        Self { websocket_tx }
    }

    pub fn get_channels(&self) -> (Sender<OrderBook>, Receiver<OrderBook>) {
        let (order_book_tx, order_book_rx) = channel::<OrderBook>(100);
        (order_book_tx, order_book_rx)
    }

    pub async fn send_message(&mut self, message: &str) {
        self.websocket_tx.send(message.into()).await.unwrap();
    }
}
