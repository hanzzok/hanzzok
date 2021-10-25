use serde::Deserialize;

use crate::Error;

use self::deserialize::HzdataDeserializer;

pub(super) use seq_access::*;

mod deserialize;
mod parse;
mod seq_access;

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    T::deserialize(&mut HzdataDeserializer {
        source: s.as_bytes(),
    })
}
