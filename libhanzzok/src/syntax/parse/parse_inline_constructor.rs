use crate::{
    core::ast::InlineConstructorNode,
    syntax::{parse::nom_ext::satisfy_transform, TokenKind},
};

use super::{
    nom_ext::{tag, HanzzokParser},
    ParseResult,
};

pub fn parse_inline_constructor(p: HanzzokParser) -> ParseResult<InlineConstructorNode> {
    let (p, number_sign) = tag(TokenKind::PunctuationNumberSign)(p)?;

    let (p, (name_token, name)) = satisfy_transform(|t| match &t.kind {
        TokenKind::Word(w) => Some(w.clone()),
        _ => None,
    })(p)?;

    Ok((
        p,
        InlineConstructorNode {
            span: number_sign.span.joined(&name_token.span),
            name,
        },
    ))
}
