use serde::Deserialize;

use crate::Error;

use self::deserialize::HzdataDeserializer;

mod deserialize;
mod parse;

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    T::deserialize(HzdataDeserializer {
        source: s.as_bytes(),
    })
}
