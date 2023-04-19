use crate::exchange::Exchange;
use crate::service::Level;
use serde::{
    de::{Error, SeqAccess},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

use serde::de::{self, Visitor};
#[derive(Deserialize, Debug)]
pub struct OrderBookBuilder<X: Exchange + std::fmt::Debug> {
    pub bids: Vec<LevelDataBuilder<X>>,
    pub asks: Vec<LevelDataBuilder<X>>,
}

#[derive(Debug)]
pub struct OrderBook {
    exchange: &'static str,
    bids: Vec<LevelData>,
    asks: Vec<LevelData>,
}

impl OrderBook {
    pub fn get_exchange_name(&self) -> &'static str {
        self.exchange
    }
    pub fn get_bids(&self) -> Vec<Level> {
        let exchange_name = self.get_exchange_name();
        self.bids
            .iter()
            .map(|bid_level_data| bid_level_data.into_level(exchange_name))
            .collect()
    }
    pub fn get_asks(&self) -> Vec<Level> {
        let exchange_name = self.get_exchange_name();
        self.asks
            .iter()
            .map(|ask_level_data| ask_level_data.into_level(exchange_name))
            .collect()
    }
}

impl<X: Exchange + std::fmt::Debug> OrderBookBuilder<X> {
    pub fn build(self) -> OrderBook {
        let OrderBookBuilder { mut bids, mut asks } = self;

        bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

        OrderBook {
            exchange: X::get_name(),
            bids: bids.into_iter().take(10).map(|b| b.build()).collect(),
            asks: asks.into_iter().take(10).map(|a| a.build()).collect(),
        }
    }
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ee7f582b5873013723596790a7993925
// https://serde.rs/string-or-struct.html
#[derive(Debug, PartialEq)]
pub struct LevelDataBuilder<X: Exchange> {
    price: f64,
    amount: f64,
    phantom: PhantomData<X>,
}

impl<X: Exchange> LevelDataBuilder<X> {
    pub fn build(self) -> LevelData {
        LevelData {
            price: self.price,
            amount: self.amount,
        }
    }
}

#[derive(Debug)]
pub struct LevelData {
    price: f64,
    amount: f64,
}

impl LevelData {
    pub fn into_level(&self, exchange_name: &'static str) -> Level {
        Level {
            exchange: exchange_name.into(),
            price: self.price,
            amount: self.amount,
        }
    }
}

impl<X: Exchange> LevelDataBuilder<X> {
    pub fn new(price: f64, amount: f64) -> Self {
        Self {
            price,
            amount,
            phantom: PhantomData,
        }
    }
}

struct LevelVisitor<X: Exchange>(PhantomData<fn() -> X>);

impl<'de, X: Exchange> Visitor<'de> for LevelVisitor<X> {
    type Value = LevelDataBuilder<X>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("An order book level")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let maybe_price = seq.next_element::<&str>()?;
        let maybe_amount = seq.next_element::<&str>()?;

        if let (Some(price), Some(amount)) = (maybe_price, maybe_amount) {
            let price = price.parse::<f64>().map_err(Error::custom)?;
            let amount = amount.parse::<f64>().map_err(Error::custom)?;
            Ok(LevelDataBuilder::new(price, amount))
        } else {
            Err(de::Error::custom("Expected a array with two elements"))
        }
    }
}

impl<'de, X: Exchange> Deserialize<'de> for LevelDataBuilder<X> {
    fn deserialize<D>(deserializer: D) -> Result<LevelDataBuilder<X>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(LevelVisitor(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binance::Binance;
    use crate::bitstamp::Bitstamp;
    use crate::test_data::{get_binance_order_book_builder, get_bitstamp_order_book_builder};

    #[test]
    fn test_deserialize_level_builder() {
        let level_builder: LevelDataBuilder<Bitstamp> =
            serde_json::from_str("[\"1.0\", \"2.0\"]").unwrap();
        assert_eq!(level_builder.price, 1.0);
        assert_eq!(level_builder.amount, 2.0);
    }

    #[test]
    fn test_level_builder() {
        let level_builder: LevelDataBuilder<Bitstamp> = LevelDataBuilder::new(1.0, 2.0);
        let level = level_builder.build();
        assert_eq!(level.price, 1.0);
        assert_eq!(level.amount, 2.0);
    }

    #[test]
    fn test_order_book_builder() {
        let order_book_builder: OrderBookBuilder<Bitstamp> = get_bitstamp_order_book_builder();

        let order_book = order_book_builder.build();

        assert_eq!(order_book.exchange, "bitstamp");
        assert_eq!(order_book.bids.len(), 10);
        assert_eq!(order_book.asks.len(), 10);

        let order_book_builder: OrderBookBuilder<Binance> = get_binance_order_book_builder();

        let order_book = order_book_builder.build();

        assert_eq!(order_book.exchange, "binance");
        assert_eq!(order_book.bids.len(), 10);
        assert_eq!(order_book.asks.len(), 10);
    }
}
