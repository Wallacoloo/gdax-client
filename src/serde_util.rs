/// Utility functions for serializing and deserializing values
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use serde::de;
use serde::de::{Deserializer, Expected, Unexpected, Visitor};
use type_name;

// Source: https://stackoverflow.com/a/44838523/216292
/// Deserialize a string from the deserializer, but then parse it as a f64.
pub fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_f64(NoStrVisitor::default())
}

/// Deserialize a string from the deserializer, but then parse it as a u64.
pub fn string_as_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_u64(NoStrVisitor::default())
}

/// Struct used to deserialize a string and parse it as some other value
#[derive(Default)]
struct NoStrVisitor<T> {
    t: PhantomData<T>,
}
impl<'de, T: FromStr> Visitor<'de> for NoStrVisitor<T> {
    type Value = T;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Self::EXPECTED.fmt(formatter)
    }
    fn visit_str<E>(self, value: &str) -> Result<T, E>
    where
        E: de::Error,
        E: 
    {
        value.parse::<T>().map_err(|_err| {
            E::invalid_value(Unexpected::Str(value),
                &Self::EXPECTED
            )
        })
    }
}

impl <T> NoStrVisitor<T> {
    const EXPECTED: NoStrExpected<T> = NoStrExpected{ t: PhantomData };
}


/// Struct to implement de::Expected for NoStrVisitor, allowing a unique error
/// message for each T if we fail to parse it.
struct NoStrExpected<T> {
    t: PhantomData<T>,
}

impl<T> de::Expected for NoStrExpected<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("a string representation of a {}", type_name::get::<T>()))
    }
}
