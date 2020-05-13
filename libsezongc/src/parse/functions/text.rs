use super::util::*;
use crate::api::{InlineObject, PlainText, Span, Spanned, Token, TokenKind};
use crate::parse::{ParseResult, Parser};

pub(crate) fn text(parser: &mut Parser<'_>) -> ParseResult<InlineObject> {
    let tokens = take_while(any::<Token, _>((
        exact(TokenKind::Text),
        exact(TokenKind::Whitespace),
    )))(parser)?;

    if tokens.is_empty() {
        return unmatched(parser);
    }

    Ok(InlineObject::PlainText(PlainText {
        span: Span::from_sequence(tokens.into_iter()).expect("Same file"),
    }))
}

pub(crate) fn fallback_text(parser: &mut Parser<'_>) -> ParseResult<InlineObject> {
    if let Ok(token) = take(parser) {
        Ok(InlineObject::PlainText(PlainText { span: token.span() }))
    } else {
        unmatched(parser)
    }
}
