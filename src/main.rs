mod binance;
mod bitstamp;
mod exchange;
mod order_book;
use crate::binance::Binance;
use crate::bitstamp::Bitstamp;
use crate::order_book::OrderBook;
use anyhow::Result;

use tokio::{join, spawn, sync::mpsc::unbounded_channel};
#[cfg(test)]
mod test_data;

#[tokio::main]
async fn main() -> Result<()> {
    let pair = "ethbtc";

    let (tx, mut rx) = unbounded_channel::<OrderBook>();

    let bitstamp = spawn(Bitstamp::get_order_book(pair, tx.clone()));
    let binance = spawn(Binance::get_order_book(pair, tx));
    let receiver = spawn(async move {
        while let Some(order_book) = rx.recv().await {
            dbg!(order_book);
        }
    });

    join!(bitstamp, binance, receiver);
    Ok(())
}
