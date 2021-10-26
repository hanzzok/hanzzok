use serde::Deserialize;

use crate::{Error, HzdataValue};

use self::deserialize::HzdataDeserializer;

pub(super) use map_access::*;
pub(super) use seq_access::*;

mod deserialize;
mod map_access;
mod parse;
mod seq_access;
mod value;
mod value_map_access;
mod value_seq_access;

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error<'a>>
where
    T: Deserialize<'a>,
{
    T::deserialize(&mut HzdataDeserializer {
        source: s.as_bytes(),
    })
}

pub fn from_value<'a, T>(value: HzdataValue) -> Result<T, Error<'a>>
where
    T: Deserialize<'a>,
{
    T::deserialize(value)
}
