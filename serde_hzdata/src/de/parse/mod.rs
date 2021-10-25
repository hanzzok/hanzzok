pub use parse_bool::parse_bool;
pub use parse_identifier::parse_identifier;
pub use parse_number::{parse_number, Number};
pub use parse_string::parse_string;
pub(super) use util::*;

mod parse_bool;
mod parse_identifier;
mod parse_number;
mod parse_string;
mod util;

pub type ParseResult<'a, T> = nom::IResult<&'a [u8], T>;
