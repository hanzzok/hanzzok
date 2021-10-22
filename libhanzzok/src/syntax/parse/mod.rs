use std::collections::HashMap;

use nom::{
    branch::alt,
    combinator::map,
    multi::{many0, many1},
};

use crate::{
    core::{
        ast::{BlockConstructorForm, HanzzokAstNode, InlineObjectNode, TextNode},
        Compiler,
    },
    syntax::{
        parse::{
            nom_ext::{tag, HanzzokParser},
            parse_inline_object::parse_inline_object,
            parse_text::parse_single_newline,
        },
        TokenKind,
    },
};

use self::{parse_block_constructor::parse_block_constructor, parse_newline::parse_newline};

use super::Token;

mod nom_ext;
mod parse_block_constructor;
mod parse_decorator_chain;
mod parse_inline_constructor;
mod parse_inline_object;
mod parse_newline;
mod parse_text;

type Error = nom::error::Error<HanzzokParser>;
type ParseResult<T> = nom::IResult<HanzzokParser, T, Error>;

pub fn parse_root(tokens: Vec<Token>, compiler: &Compiler) -> Vec<HanzzokAstNode> {
    let p = HanzzokParser::new(tokens, {
        let mut map = HashMap::new();

        for (key, rule) in &compiler.block_constructor_rules {
            map.insert(key.clone(), rule.form());
        }

        map
    });
    let nodes: Vec<_> = many0(alt((
        map(parse_block_constructor, |node| {
            vec![HanzzokAstNode::BlockConstructor(node)]
        }),
        map(many1(parse_inline_object), |nodes| {
            nodes
                .into_iter()
                .map(HanzzokAstNode::InlineObject)
                .collect()
        }),
        map(parse_newline, |node| vec![node]),
        map(parse_single_newline, |node| {
            vec![HanzzokAstNode::InlineObject(InlineObjectNode::Text(node))]
        }),
    )))(p)
    .map(|(_, vec)| vec)
    .unwrap_or_else(|_| Vec::new())
    .into_iter()
    .flatten()
    .collect();

    let mut result = Vec::new();

    let mut last = None;
    for node in nodes {
        last = Some(match (last, node) {
            (
                Some(HanzzokAstNode::InlineObject(InlineObjectNode::Text(l))),
                HanzzokAstNode::InlineObject(InlineObjectNode::Text(r)),
            ) => HanzzokAstNode::InlineObject(InlineObjectNode::Text(l.merged_with(&r))),
            (Some(old), new) => {
                result.push(old);
                new
            }
            (_, new) => new,
        })
    }
    if let Some(node) = last {
        result.push(node);
    }

    result
}
