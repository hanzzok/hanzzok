use nom::{combinator::map, sequence::tuple};

use crate::{core::ast::HanzzokAstNode, syntax::TokenKind};

use super::{
    nom_ext::{tag, HanzzokParser},
    ParseResult,
};

pub fn parse_newline(p: HanzzokParser) -> ParseResult<HanzzokAstNode> {
    map(
        tuple((tag(TokenKind::VerticalSpace), tag(TokenKind::VerticalSpace))),
        |(l, r)| HanzzokAstNode::Newline(vec![l, r]),
    )(p)
}
