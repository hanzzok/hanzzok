use crate::api::{InlineObject, LineColumn, ParserResult, ParserSpan, PlainText, Span, Spanned};
use nom::{bytes::complete::tag, character::complete::anychar};
use std::iter::once;

pub(crate) fn escape(s: ParserSpan<'_>) -> ParserResult<'_, InlineObject> {
    let start = LineColumn::create_from(&s);
    let (s, _) = tag("\\")(s)?;
    let (s, text) = anychar::<_, ()>(s)
        .map(|(s, c)| (s, once(c).collect()))
        .unwrap_or((s, "\\".to_owned()));
    let end = LineColumn::create_from(&s);
    Ok((
        s,
        Spanned {
            value: InlineObject::PlainText(PlainText { text }),
            span: Span { start, end },
        },
    ))
}
