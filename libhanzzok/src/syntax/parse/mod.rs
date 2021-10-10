use std::collections::HashMap;

use nom::{combinator::map, multi::many0};

use crate::{
    core::ast::HanzzokAstNode,
    syntax::parse::{nom_ext::HanzzokParser, parse_inline_object::parse_inline_object},
};

use super::Token;

mod nom_ext;
mod parse_decorator_chain;
mod parse_inline_constructor;
mod parse_inline_object;
mod parse_text;

type Error = nom::error::Error<HanzzokParser>;
type ParseResult<T> = nom::IResult<HanzzokParser, T, Error>;

pub fn parse_root(tokens: Vec<Token>) -> Vec<HanzzokAstNode> {
    let p = HanzzokParser {
        // TODO
        block_constructors: HashMap::new(),
        offset: 0,
        tokens,
    };
    many0(map(parse_inline_object, HanzzokAstNode::InlineObject))(p)
        .map(|(_, vec)| vec)
        .unwrap_or_else(|_| Vec::new())
}
