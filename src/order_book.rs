use crate::exchange::Exchange;
use serde::{
    de::{Error, SeqAccess},
    Deserialize, Deserializer,
};
use std::{fmt, marker::PhantomData};

use serde::de::{self, Visitor};

#[derive(Deserialize, Debug)]
pub struct OrderBook<X: Exchange> {
    pub bids: Vec<Level<X>>,
    pub asks: Vec<Level<X>>,
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ee7f582b5873013723596790a7993925
// https://serde.rs/string-or-struct.html
#[derive(Debug)]
pub struct Level<X: Exchange> {
    exchange: &'static str,
    price: f64,
    amount: f64,
    phantom: PhantomData<X>,
}

impl<X: Exchange> Level<X> {
    pub fn new(exchange: &'static str, price: f64, amount: f64) -> Self {
        Self {
            exchange,
            price,
            amount,
            phantom: PhantomData,
        }
    }
}

struct LevelVisitor<X: Exchange>(PhantomData<fn() -> X>);

impl<'de, X: Exchange> Visitor<'de> for LevelVisitor<X> {
    type Value = Level<X>;

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
            Ok(Level::new(X::get_name(), price, amount))
        } else {
            Err(de::Error::custom("Expected a array with two elements"))
        }
    }
}

impl<'de, X: Exchange> Deserialize<'de> for Level<X> {
    fn deserialize<D>(deserializer: D) -> Result<Level<X>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(LevelVisitor(PhantomData))
    }
}
