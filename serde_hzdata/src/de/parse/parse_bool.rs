use nom::{branch::alt, bytes::complete::tag, combinator::map};

use super::{skip_whitespaces, ParseResult};

pub fn parse_bool(s: &[u8]) -> ParseResult<bool> {
    let (s, _) = skip_whitespaces(s)?;

    alt((map(tag(b"true"), |_| true), map(tag(b"false"), |_| false)))(s)
}
