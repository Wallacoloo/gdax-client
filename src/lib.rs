#[macro_use]
extern crate serde_derive;

extern crate base64;
extern crate chrono;
extern crate crypto;
extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate time;
extern crate type_name;
extern crate uuid;
extern crate ws;

use std::fmt;

pub mod public;
pub mod private;
pub mod websocket;
mod serde_util;

pub use public::Client as PublicClient;
pub use private::Client as PrivateClient;
pub use websocket::WebsocketClient as WebsocketClient;

pub use private::NewOrder;
pub use private::SizeOrFunds::{self, Funds, Size};

#[derive(Debug, Deserialize)]
pub struct ApiError {
    message: String
}

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Http(hyper::Error),
    InvalidSecretKey,
    Json(serde_json::Error),
    InvalidArgument(String)
}

impl std::convert::From<base64::DecodeError> for Error {
    fn from(_: base64::DecodeError) -> Error {
        // Only time we get a base64 error is when decoding secret key
        Error::InvalidSecretKey
    }
}

impl std::convert::From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Side {
    Buy,
    Sell
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Side::Buy => write!(f, "Buy"),
            Side::Sell => write!(f, "Sell")
        }
    }
}

// We manually implement Serialize for Side here
// because the default encoding/decoding scheme that derive
// gives us isn't the straightforward mapping unfortunately
impl serde::Serialize for Side {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        match self {
            &Side::Buy => serializer.serialize_str("buy"),
            &Side::Sell => serializer.serialize_str("sell")
        }
    }
}

// We manually implement Deserialize for Side here
// because the default encoding/decoding scheme that derive
// gives us isn't the straightforward mapping unfortunately
impl<'de> serde::Deserialize<'de> for Side {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        struct SideVisitor;

        impl<'de> serde::de::Visitor<'de> for SideVisitor {
            type Value = Side;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("side must be either `buy` or `sell`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: serde::de::Error {
                match &*v.to_lowercase() {
                    "buy" => Ok(Side::Buy),
                    "sell" => Ok(Side::Sell),
                    _ => Err(E::custom("side must be either `buy` or `sell`"))
                }
            }
        }

        deserializer.deserialize_any(SideVisitor)
    }
}

