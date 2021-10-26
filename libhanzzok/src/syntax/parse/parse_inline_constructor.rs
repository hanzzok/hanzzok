use nom::multi::many1;

use crate::{
    core::ast::InlineConstructorNode,
    syntax::{parse::nom_ext::satisfy_transform, Token, TokenKind},
};

use super::{
    nom_ext::{tag, HanzzokParser},
    parse_hzdata::parse_hzdata_paired,
    ParseResult,
};

fn parse_inline_constructor_params(p: HanzzokParser) -> ParseResult<Vec<Token>> {
    parse_hzdata_paired(
        TokenKind::PunctuationLeftParenthesis,
        TokenKind::PunctuationRightParenthesis,
        true,
    )(p)
}

pub fn parse_inline_constructor(p: HanzzokParser) -> ParseResult<InlineConstructorNode> {
    let tt = p.create_tracker();
    let (p, _) = tag(TokenKind::PunctuationNumberSign)(p)?;

    let (p, name) = many1(satisfy_transform(|t| match &t.kind {
        TokenKind::Word(w) => Some(w.clone()),
        TokenKind::PunctuationHyphenMinus => Some("-".to_owned()),
        _ => None,
    }))(p)?;

    let name: String = name.into_iter().map(|(_, name)| name).collect();

    let (p, params) = parse_inline_constructor_params(p)?;

    let tokens = tt.end(&p);

    Ok((
        p,
        InlineConstructorNode {
            name,
            params,
            tokens,
        },
    ))
}
