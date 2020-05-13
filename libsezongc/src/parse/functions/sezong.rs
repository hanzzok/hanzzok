use super::{block_constructor, inline_object, util::*};
use crate::api::{Ast, Span};
use crate::parse::{ParseError, ParseResult, Parser};

pub(crate) fn sezong(parser: &mut Parser<'_>) -> ParseResult<Vec<Ast>> {
    let result = take_while(any((block_constructor, into(inline_object))))(parser)?;

    if parser.has_next() {
        ParseError::Unused(
            Span::from_sequence(parser.tokens[parser.cursor..].to_vec().into_iter())
                .expect("Not empty, same file"),
        )
        .into()
    } else {
        Ok(result)
    }
}
