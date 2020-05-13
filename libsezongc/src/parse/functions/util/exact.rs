use super::*;
use crate::api::{Token, TokenKind};
use crate::parse::{ParseResult, Parser};

pub(crate) trait ExactFilter {
    fn filter(&mut self, token: &Token) -> bool;
}

impl ExactFilter for TokenKind {
    fn filter(&mut self, token: &Token) -> bool {
        token.kind == *self
    }
}

impl<F> ExactFilter for F
where
    F: FnMut(&Token) -> bool,
{
    fn filter(&mut self, token: &Token) -> bool {
        self(token)
    }
}

fn exact_raw<F>(mut filter: F, required: bool) -> impl FnMut(&mut Parser<'_>) -> ParseResult<Token>
where
    F: ExactFilter,
{
    move |parser| {
        let cursor = parser.cursor;
        let token = take(parser)?;
        if filter.filter(&token) {
            Ok(token)
        } else {
            parser.cursor = cursor;
            if required {
                unexpected(parser)
            } else {
                unmatched(parser)
            }
        }
    }
}

pub(crate) fn exact<F>(filter: F) -> impl FnMut(&mut Parser<'_>) -> ParseResult<Token>
where
    F: ExactFilter,
{
    exact_raw(filter, false)
}

pub(crate) fn exact_require<F>(filter: F) -> impl FnMut(&mut Parser<'_>) -> ParseResult<Token>
where
    F: ExactFilter,
{
    exact_raw(filter, true)
}
