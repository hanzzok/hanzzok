use nom::bytes::complete::take_while;

use super::ParseResult;

pub fn skip_whitespaces(s: &[u8]) -> ParseResult<&[u8]> {
    take_while(|c| matches!(c, b' ' | b'\t' | b'\r' | b'\n'))(s)
}
