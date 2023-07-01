use crate::{exchange::Exchange, proto::Level};
use serde::{
    de::{self, Error, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};
#[derive(Deserialize)]
pub struct OrderBookBuilder<X: Exchange> {
    pub bids: Vec<LevelBuilder<X>>,
    pub asks: Vec<LevelBuilder<X>>,
}

#[derive(Debug)]
pub struct OrderBook {
    exchange: &'static str,
    bids: Vec<Level>,
    asks: Vec<Level>,
}

impl OrderBook {
    pub fn get_exchange_name(&self) -> &'static str {
        self.exchange
    }
    pub fn get_levels(&self) -> (Vec<Level>, Vec<Level>) {
        (self.bids.clone(), self.asks.clone())
    }
}

impl<X: Exchange> OrderBookBuilder<X> {
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
#[derive(PartialEq)]
pub struct LevelBuilder<X: Exchange> {
    price: f64,
    amount: f64,
    phantom: PhantomData<X>,
}

impl<X: Exchange> LevelBuilder<X> {
    pub fn build(self) -> Level {
        Level {
            exchange: X::get_name().into(),
            price: self.price,
            amount: self.amount,
        }
    }
}

impl<X: Exchange> LevelBuilder<X> {
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
    type Value = LevelBuilder<X>;

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
            Ok(LevelBuilder::new(price, amount))
        } else {
            Err(de::Error::custom("Expected a array with two elements"))
        }
    }
}

impl<'de, X: Exchange> Deserialize<'de> for LevelBuilder<X> {
    fn deserialize<D>(deserializer: D) -> Result<LevelBuilder<X>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(LevelVisitor(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        binance::Binance,
        bitstamp::Bitstamp,
        test_data::{get_binance_order_book_builder, get_bitstamp_order_book_builder},
    };

    #[test]
    fn test_deserialize_level_builder() {
        let level_builder: LevelBuilder<Bitstamp> =
            serde_json::from_str("[\"1.0\", \"2.0\"]").unwrap();
        assert_eq!(level_builder.price, 1.0);
        assert_eq!(level_builder.amount, 2.0);
    }

    #[test]
    fn test_level_builder() {
        let level_builder: LevelBuilder<Bitstamp> = LevelBuilder::new(1.0, 2.0);
        let level = level_builder.build();
        assert_eq!(level.price, 1.0);
        assert_eq!(level.amount, 2.0);
    }

    #[test]
    fn test_order_book_builder() {
        // Bitstamp
        let order_book_builder: OrderBookBuilder<Bitstamp> = get_bitstamp_order_book_builder();

        let order_book = order_book_builder.build();

        assert_eq!(order_book.exchange, "bitstamp");

        assert_eq!(order_book.bids.len(), 10);
        assert_eq!(order_book.asks.len(), 10);

        // highest price first for bids
        assert_eq!(order_book.bids[0].price, 0.06842268);
        assert_eq!(order_book.bids[9].price, 0.06839848);

        // lowest price first for asks
        assert_eq!(order_book.asks[0].price, 0.06843007);
        assert_eq!(order_book.asks[9].price, 0.06847844);

        // Binance
        let order_book_builder: OrderBookBuilder<Binance> = get_binance_order_book_builder();

        let order_book = order_book_builder.build();

        assert_eq!(order_book.exchange, "binance");

        assert_eq!(order_book.bids.len(), 10);
        assert_eq!(order_book.asks.len(), 10);

        // highest price first for bids
        assert_eq!(order_book.bids[0].price, 0.068426);
        assert_eq!(order_book.bids[9].price, 0.068416);

        // lowest price first for asks
        assert_eq!(order_book.asks[0].price, 0.068427);
        assert_eq!(order_book.asks[9].price, 0.068438);
    }
}
