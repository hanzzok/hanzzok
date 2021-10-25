use nom::{
    branch::alt,
    bytes::complete::{take_while, take_while1},
    combinator::map,
    sequence::tuple,
};

use crate::de::parse::skip_whitespaces;

use super::{parse_string, ParseResult};

pub fn parse_identifier(s: &[u8]) -> ParseResult<String> {
    let (s, _) = skip_whitespaces(s)?;
    alt((
        parse_string,
        map(
            tuple((
                take_while1(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z')),
                take_while(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9')),
            )),
            |(l, r)| String::from_utf8([l, r].concat()).unwrap().to_string(),
        ),
    ))(s)
}
