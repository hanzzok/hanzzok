use crate::parse::{ParseResult, Parser};

pub(crate) fn into<T, I, F>(mut f: F) -> impl FnMut(&mut Parser<'_>) -> ParseResult<T>
where
    I: Into<T>,
    F: FnMut(&mut Parser<'_>) -> ParseResult<I>,
{
    move |parser| f(parser).map(|value| value.into())
}
