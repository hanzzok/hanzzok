use super::base::is_text_char;
use crate::api::parse_prelude::*;
use nom::{bytes::complete::take_while1, character::complete::anychar};

pub(crate) fn text(s: ParserSpan<'_>) -> ParserResult<'_, InlineObject> {
    Spanned::wrap(|s| {
        let (s, text) = take_while1(is_text_char)(s)?;
        Ok((
            s,
            InlineObject::PlainText(PlainText {
                text: text.fragment().clone().to_owned(),
            }),
        ))
    })(s)
}

pub(crate) fn fallback_text(s: ParserSpan<'_>) -> ParserResult<'_, InlineObject> {
    Spanned::wrap(|s| {
        let (s, v) = anychar(s)?;
        Ok((
            s,
            InlineObject::PlainText(PlainText {
                text: v.to_string(),
            }),
        ))
    })(s)
}
