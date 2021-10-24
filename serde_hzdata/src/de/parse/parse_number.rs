use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::is_digit,
    combinator::{consumed, opt, value},
    sequence::{preceded, tuple},
};

use super::{skip_whitespaces, ParseResult};

pub enum Number {
    I64(i64),
    F64(f64),
}

pub fn parse_number(s: &[u8]) -> ParseResult<Number> {
    let (s, _) = skip_whitespaces(s)?;

    let (s, res) = opt(alt((
        value(f64::INFINITY, tag(b"inf")),
        value(f64::NEG_INFINITY, tag(b"-inf")),
        value(f64::NAN, tag("nan")),
    )))(s)?;

    if let Some(res) = res {
        return Ok((s, Number::F64(res)));
    }

    let (s, sign) = opt(alt((tag(b"+"), tag(b"-"))))(s)?;

    let (s, integer) = take_while1(is_digit)(s)?;

    let (s, decimal) = opt(consumed(tuple((
        opt(preceded(tag(b"."), take_while1(is_digit))),
        opt(preceded(
            alt((tag(b"e"), tag(b"E"))),
            tuple((opt(alt((tag(b"+"), tag(b"-")))), take_while1(is_digit))),
        )),
    ))))(s)?;

    let res = match decimal {
        None | Some((_, (None, None))) => Number::I64(
            String::from_utf8([sign.unwrap_or(&[]), integer].concat())
                .unwrap()
                .parse()
                .unwrap(),
        ),
        Some((decimal, _)) => Number::F64(
            String::from_utf8([sign.unwrap_or(&[]), integer, decimal].concat())
                .unwrap()
                .parse()
                .unwrap(),
        ),
    };

    Ok((s, res))
}
