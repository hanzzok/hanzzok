use nom::{branch::alt, combinator::map, multi::many0};

use crate::{
    core::ast::HanzzokAstNode,
    syntax::parse::{
        nom_ext::HanzzokParser,
        parse_text::{parse_escaped_text, parse_fallback_text},
    },
};

use super::Token;

mod nom_ext;
mod parse_decorator_chain;
mod parse_text;

type Error = nom::error::Error<HanzzokParser>;
type ParseResult<T> = nom::IResult<HanzzokParser, T, Error>;

pub fn parse_root(tokens: Vec<Token>) -> Vec<HanzzokAstNode> {
    let p = HanzzokParser {
        // TODO
        block_constructor_names: vec![],
        tokens,
    };
    many0(alt((
        map(parse_escaped_text, HanzzokAstNode::Text),
        map(parse_fallback_text, HanzzokAstNode::Text),
    )))(p)
    .map(|(_, vec)| vec)
    .unwrap_or_else(|_| Vec::new())
}
