use std::collections::HashMap;

use nom::{branch::alt, combinator::map, multi::many0};

use crate::{
    core::ast::{BlockConstructorForm, HanzzokAstNode},
    syntax::parse::{nom_ext::HanzzokParser, parse_inline_object::parse_inline_object},
};

use self::parse_block_constructor::parse_block_constructor;

use super::Token;

mod nom_ext;
mod parse_block_constructor;
mod parse_decorator_chain;
mod parse_inline_constructor;
mod parse_inline_object;
mod parse_text;

type Error = nom::error::Error<HanzzokParser>;
type ParseResult<T> = nom::IResult<HanzzokParser, T, Error>;

pub fn parse_root(tokens: Vec<Token>) -> Vec<HanzzokAstNode> {
    let p = HanzzokParser::new(tokens, {
        let mut map = HashMap::new();

        map.insert("#".to_string(), BlockConstructorForm::Shortened);
        map.insert("##".to_string(), BlockConstructorForm::Shortened);
        map.insert("###".to_string(), BlockConstructorForm::Shortened);
        map.insert("####".to_string(), BlockConstructorForm::Shortened);
        map.insert("#####".to_string(), BlockConstructorForm::Shortened);
        map.insert("######".to_string(), BlockConstructorForm::Shortened);

        map
    });
    many0(alt((
        map(parse_block_constructor, HanzzokAstNode::BlockConstructor),
        map(parse_inline_object, HanzzokAstNode::InlineObject),
    )))(p)
    .map(|(_, vec)| vec)
    .unwrap_or_else(|_| Vec::new())
}
