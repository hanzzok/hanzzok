use crate::syntax::TokenKind;

use super::{
    nom_ext::{tag, HanzzokParser},
    ParseResult,
};

pub fn parse_decorator_chain(p: HanzzokParser) -> ParseResult<()> {
    let (p, open) = tag(TokenKind::PunctuationLeftSquareBracket)(p)?;

    Ok((p, ()))
}
