use super::util::*;
use crate::api::{InlineObject, PlainText, Spanned, TokenKind};
use crate::parse::{ParseResult, Parser};

pub(crate) fn escape(parser: &mut Parser<'_>) -> ParseResult<InlineObject> {
    let span = {
        let solidus = exact(TokenKind::PunctuationReverseSolidus)(parser)?;
        match take(parser) {
            Ok(token) => solidus.span().joined(&token.span()).expect("Same file"),
            _ => solidus.span(),
        }
    };
    Ok(InlineObject::PlainText(PlainText { span }))
}
