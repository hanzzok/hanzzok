use crate::api::Token;
use crate::parse::{ParseError, ParseResult, Parser};

pub(crate) fn unexpected<T>(parser: &mut Parser<'_>) -> ParseResult<T> {
    if let Some(token) = parser.tokens.get(parser.cursor) {
        unexpected_token(token.clone())
    } else {
        unexpected_eof(parser)
    }
}

pub(crate) fn unexpected_token<T>(token: Token) -> ParseResult<T> {
    ParseError::UnexpectedToken(token).into()
}

pub(crate) fn unexpected_eof<T>(parser: &mut Parser<'_>) -> ParseResult<T> {
    ParseError::UnexpectedEof(parser.span_last_used()).into()
}
