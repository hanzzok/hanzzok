use super::*;
use crate::api::Token;
use crate::parse::{ParseResult, Parser};

pub(crate) fn not<F>(mut function: F) -> impl FnMut(&mut Parser<'_>) -> ParseResult<Token>
where
    F: FnMut(&mut Parser<'_>) -> ParseResult<Token>,
{
    move |parser| {
        let cursor = parser.cursor;
        match function(parser) {
            Ok(token) => unmatched_token(&token),
            Err(_) => {
                let result = parser.tokens[cursor].clone();
                parser.cursor = cursor + 1;
                Ok(result)
            }
        }
    }
}
