use crate::{
    core::ast::TextNode,
    syntax::{parse::nom_ext::tag, TokenKind},
};

use super::{
    nom_ext::{any, HanzzokParser},
    ParseResult,
};

pub fn parse_fallback_text(p: HanzzokParser) -> ParseResult<TextNode> {
    let (p, token) = any(p)?;

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
