use super::*;
use crate::api::{Spanned, Token};
use crate::parse::{ParseError, ParseResult, Parser};

pub(crate) fn unmatched<T>(parser: &mut Parser<'_>) -> ParseResult<T> {
    parser
        .last_used()
        .map(unmatched_token)
        .unwrap_or_else(|| unexpected_eof(parser))
}

pub(crate) fn unmatched_token<T>(token: &Token) -> ParseResult<T> {
    ParseError::Unmatched(token.span()).into()
}
