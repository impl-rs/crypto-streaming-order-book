mod binance;
mod bitstamp;
mod exchange;
mod order_book;
use crate::binance::Binance;
use crate::bitstamp::Bitstamp;
use anyhow::Result;
use tokio::join;

#[tokio::main]
async fn main() -> Result<()> {
    let pair = "ethbtc";
    join!(
        Bitstamp::get_order_book(pair),
        Binance::get_order_book(pair)
    );

    Ok(())
}
