use super::{decorator, escape, fallback_text, inline_constructor, text};
use crate::api::parse_prelude::*;
use nom::branch::alt;

pub(crate) fn inline_object(s: ParserSpan<'_>) -> ParserResult<'_, Ast> {
    let (s, inline_object) = alt((escape, text, inline_constructor, decorator, fallback_text))(s)?;

    Ok((
        s,
        inline_object.map_wrapped(|inline_object| Ast::InlineObject(inline_object)),
    ))
}
