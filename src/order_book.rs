use crate::exchange::Exchange;
use serde::{de::Error, Deserialize, Deserializer};
use serde_json::Value;

macro_rules! value_to_float {
    ( $value:expr, $deserializer:ident ) => {{
        match $value {
            Value::String(string) => string.parse().map_err(Error::custom),
            Value::Number(number) => number.as_f64().ok_or(Error::custom("not a float")),
            // It was necessary to add the deserializer error type here
            _ => Err::<f64, $deserializer::Error>(Error::custom("wrong type")),
        }
    }};
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ee7f582b5873013723596790a7993925
// https://serde.rs/string-or-struct.html
#[derive(Debug, Deserialize)]
pub struct BidAsk {
    exchange: String,
    price: f64,
    amount: f64,
}

impl BidAsk {
    pub fn new(exchange: String, price: f64, amount: f64) -> Self {
        Self {
            exchange,
            price,
            amount,
        }
    }
}

// I considered doing a custom deserializer, but that might be overkill for this
pub fn deserialize_bid_ask<'de, D: Deserializer<'de>, X: Exchange>(
    deserializer: D,
) -> Result<Vec<BidAsk>, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::Array(bid_asks_vector) => bid_asks_vector
            .iter()
            .filter_map(|price_amount_vector_value| {
                if let Value::Array(price_amount_vector) = price_amount_vector_value {
                    let bid_ask = BidAsk::new(
                        X::get_name(),
                        value_to_float!(&price_amount_vector[0], D).unwrap(),
                        value_to_float!(&price_amount_vector[1], D).unwrap(),
                    );
                    Some(bid_ask)
                } else {
                    None
                }
            })
            .collect(),

        _ => return Err(Error::custom("wrong type")),
    })
}
