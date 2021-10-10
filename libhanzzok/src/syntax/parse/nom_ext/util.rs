use nom::{
    branch::alt,
    error::{ErrorKind, ParseError},
    multi::many0,
    Err, IResult, InputIter, InputTake, Slice,
};

use crate::syntax::{parse::ParseResult, Token, TokenKind};

use super::HanzzokParser;

#[inline(always)]
pub fn err_kind<T, Error: ParseError<HanzzokParser>>(
    i: HanzzokParser,
    kind: ErrorKind,
) -> IResult<HanzzokParser, T, Error> {
    Err(Err::Error(Error::from_error_kind(i, kind)))
}

#[inline(always)]
pub fn err_tag<T, Error: ParseError<HanzzokParser>>(
    i: HanzzokParser,
) -> IResult<HanzzokParser, T, Error> {
    err_kind(i, ErrorKind::Tag)
}

pub fn satisfy<F, Error: ParseError<HanzzokParser>>(
    cond: F,
) -> impl Fn(HanzzokParser) -> IResult<HanzzokParser, Token, Error>
where
    F: Fn(&Token) -> bool,
{
    move |i| match (i).iter_elements().next().map(|t| {
        let b = cond(&t);
        (t, b)
    }) {
        Some((t, true)) => Ok((i.slice(1..), t)),
        _ => err_kind(i, ErrorKind::Satisfy),
    }
}

pub fn satisfy_transform<F, U, Error: ParseError<HanzzokParser>>(
    cond: F,
) -> impl Fn(HanzzokParser) -> IResult<HanzzokParser, (Token, U), Error>
where
    F: Fn(&Token) -> Option<U>,
{
    move |i| match (i).iter_elements().next().map(|t| {
        let b = cond(&t);
        (t, b)
    }) {
        Some((t, Some(res))) => Ok((i.slice(1..), (t, res))),
        _ => err_kind(i, ErrorKind::Satisfy),
    }
}

pub fn any<Error: ParseError<HanzzokParser>>(
    i: HanzzokParser,
) -> IResult<HanzzokParser, Token, Error> {
    match i.slice_index(1) {
        Ok(index) => {
            let (i, part) = i.take_split(index);
            Ok((i, part.tokens[0].clone()))
        }
        Err(_needed) => err_kind(i, ErrorKind::Eof),
    }
}

pub fn tag<Error: ParseError<HanzzokParser>>(
    tag: TokenKind,
) -> impl Fn(HanzzokParser) -> IResult<HanzzokParser, Token, Error> {
    move |i| match i.iter_elements().next().map(|t| {
        let b = t.kind == tag;
        (t, b)
    }) {
        Some((t, true)) => Ok((i.slice(1..), t)),
        _ => err_kind(i, ErrorKind::Tag),
    }
}

pub fn skip_any_spaces(p: HanzzokParser) -> ParseResult<()> {
    let (p, _) = many0(alt((
        tag(TokenKind::HorizontalSpace),
        tag(TokenKind::VerticalSpace),
    )))(p)?;

    Ok((p, ()))
}
