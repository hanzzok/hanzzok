use nom::{
    branch::alt,
    combinator::{fail, map, not},
    multi::many1,
    sequence::{preceded, tuple},
};

use crate::{
    core::ast::TextNode,
    syntax::{parse::nom_ext::tag, TokenKind},
};

use super::{
    nom_ext::{any, satisfy, HanzzokParser},
    ParseResult,
};

pub fn parse_text(p: HanzzokParser) -> ParseResult<TextNode> {
    map(
        many1(alt((
            satisfy(|t| matches!(t.kind, TokenKind::Word(_) | TokenKind::PunctuationsOther(_))),
            preceded(
                not(tuple((
                    tag(TokenKind::VerticalSpace),
                    tag(TokenKind::VerticalSpace),
                ))),
                tag(TokenKind::VerticalSpace),
            ),
            tag(TokenKind::HorizontalSpace),
        ))),
        |tokens| TextNode {
            tokens: tokens.into_iter().map(|t| (t, true)).collect(),
        },
    )(p)
}

pub fn parse_fallback_text(p: HanzzokParser) -> ParseResult<TextNode> {
    let (p, token) = any(p)?;

    if token.kind == TokenKind::VerticalSpace {
        return fail(p);
    }

    Ok((
        p,
        TextNode {
            tokens: vec![(token, true)],
        },
    ))
}

pub fn parse_escaped_text(p: HanzzokParser) -> ParseResult<TextNode> {
    let (p, backslash) = tag(TokenKind::PunctuationReverseSolidus)(p)?;
    let (p, token) = any(p)?;

    Ok((
        p,
        TextNode {
            tokens: vec![(backslash, false), (token, true)],
        },
    ))
}
