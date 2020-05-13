use super::*;
use crate::api::Token;
use crate::parse::{ParseResult, Parser};

pub(crate) fn take(parser: &mut Parser<'_>) -> ParseResult<Token> {
    if let Some(token) = parser.tokens.get(parser.cursor) {
        parser.cursor += 1;
        Ok(token.clone())
    } else {
        unexpected_eof(parser)
    }
}
