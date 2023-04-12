mod binance;
mod bitstamp;
use crate::binance::Binance;
use crate::bitstamp::Bitstamp;
use anyhow::Result;
use tokio::join;

#[tokio::main]
async fn main() -> Result<()> {
    join!(Bitstamp::get_order_book(), Binance::get_order_book());

    Ok(())
}
