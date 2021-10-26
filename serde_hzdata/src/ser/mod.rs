use serde::Serialize;

use crate::{Error, HzdataValue};

use self::value::HzdataValueSerialize;

mod serialize;
mod serialize_map;
mod serialize_seq;
mod value;
mod value_map;
mod value_seq;

pub fn to_value<'a, T>(value: T) -> Result<HzdataValue, Error<'static>>
where
    T: Serialize,
{
    value.serialize(HzdataValueSerialize)
}
