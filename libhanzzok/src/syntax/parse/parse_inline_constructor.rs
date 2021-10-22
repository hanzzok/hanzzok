use crate::{
    core::ast::InlineConstructorNode,
    syntax::{parse::nom_ext::satisfy_transform, TokenKind},
};

use super::{
    nom_ext::{tag, HanzzokParser},
    ParseResult,
};

pub fn parse_inline_constructor(p: HanzzokParser) -> ParseResult<InlineConstructorNode> {
    let tt = p.create_tracker();
    let (p, _) = tag(TokenKind::PunctuationNumberSign)(p)?;

    let (p, (_, name)) = satisfy_transform(|t| match &t.kind {
        TokenKind::Word(w) => Some(w.clone()),
        _ => None,
    })(p)?;

    let tokens = tt.end(&p);

    Ok((p, InlineConstructorNode { name, tokens }))
}
