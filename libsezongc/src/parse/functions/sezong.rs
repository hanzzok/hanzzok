use super::{block_constructor, inline_object};
use crate::api::parse_prelude::*;
use nom::{branch::alt, multi::many0};

pub(crate) fn sezong(s: ParserSpan<'_>) -> Vec<Spanned<Ast>> {
    many0(alt((block_constructor, inline_object)))(s)
        .map(|(_, v)| v)
        .unwrap_or_else(|_| Vec::new())
}
