use nom::{
    branch::alt,
    combinator::map,
    multi::{many0, many1},
};

use crate::{
    core::{
        ast::{HanzzokAstNode, InlineObjectNode},
        Compiler,
    },
    syntax::parse::{
        nom_ext::HanzzokParser, parse_inline_object::parse_inline_object,
        parse_text::parse_single_newline,
    },
};

use self::{parse_block_constructor::parse_block_constructor, parse_newline::parse_newline};

use super::Token;

mod nom_ext;
mod parse_block_constructor;
mod parse_decorator_chain;
mod parse_hzdata;
mod parse_inline_constructor;
mod parse_inline_object;
mod parse_newline;
mod parse_text;

type Error = nom::error::Error<HanzzokParser>;
type ParseResult<T> = nom::IResult<HanzzokParser, T, Error>;

pub fn parse_root(tokens: Vec<Token>, compiler: &Compiler) -> Vec<HanzzokAstNode> {
    let p = HanzzokParser::new(tokens, &compiler.block_constructor_rules);
    many0(alt((
        map(parse_block_constructor, |node| {
            vec![HanzzokAstNode::BlockConstructor(node)]
        }),
        map(many1(parse_inline_object), |nodes| {
            vec![HanzzokAstNode::InlineObjectBlock(nodes)]
        }),
        map(parse_newline, |node| vec![node]),
        map(parse_single_newline, |node| {
            vec![HanzzokAstNode::InlineObjectBlock(vec![
                InlineObjectNode::Text(node),
            ])]
        }),
    )))(p)
    .map(|(_, vec)| vec)
    .unwrap_or_else(|_| Vec::new())
    .into_iter()
    .flatten()
    .collect()
}
