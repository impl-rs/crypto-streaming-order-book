mod bitstamp;
use crate::bitstamp::Bitstamp;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    Bitstamp::get_order_book().await?;
    Ok(())
}
