use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take, take_while_m_n},
    combinator::{map, not, opt, recognize, value},
    multi::{many0, many_m_n},
    sequence::{delimited, preceded, tuple},
};
use serde::__private::from_utf8_lossy;

use super::{skip_whitespaces, ParseResult};

pub fn parse_string(s: &[u8]) -> ParseResult<String> {
    let (s, _) = skip_whitespaces(s)?;

    let (s, res) = map(
        delimited(
            tag(b"\""),
            many0(preceded(
                not(tag(b"\"")),
                alt((
                    value(b"\'".to_vec(), tag(b"\\\'")),
                    value(b"\"".to_vec(), tag(b"\\\"")),
                    map(
                        tuple((
                            tag(b"\\x"),
                            take_while_m_n(1, 1, |c| matches!(c, b'0'..=b'7')),
                            take_while_m_n(
                                1,
                                1,
                                |c| matches!(c, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F'),
                            ),
                        )),
                        |(_, a, b): (_, &[u8], &[u8])| {
                            vec![
                                u8::from_str_radix(&String::from_utf8_lossy(&[a[0], b[0]]), 16)
                                    .unwrap(),
                            ]
                        },
                    ),
                    value(b"\n".to_vec(), tag(b"\\n")),
                    value(b"\r".to_vec(), tag(b"\\r")),
                    value(b"\t".to_vec(), tag(b"\\t")),
                    value(b"\\".to_vec(), tag(b"\\\\")),
                    value(b"\0".to_vec(), tag(b"\\0")),
                    map(
                        delimited(
                            tag(b"\\u{"),
                            take_while_m_n(
                                1,
                                6,
                                |c| matches!(c, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F'),
                            ),
                            tag(b"}"),
                        ),
                        |s: &[u8]| {
                            char::from_u32(
                                u32::from_str_radix(&String::from_utf8_lossy(s), 16).unwrap(),
                            )
                            .unwrap()
                            .to_string()
                            .as_bytes()
                            .to_vec()
                        },
                    ),
                )),
            )),
            tag(b"\""),
        ),
        |res| String::from_utf8(res.into_iter().flatten().collect()).unwrap(),
    )(s)?;

    Ok((s, res))
}
