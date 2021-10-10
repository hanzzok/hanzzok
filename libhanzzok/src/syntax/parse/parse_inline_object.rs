use nom::{branch::alt, combinator::map};

use crate::core::ast::InlineObjectNode;

use super::{
    nom_ext::HanzzokParser,
    parse_decorator_chain::parse_decorator_chain,
    parse_text::{parse_escaped_text, parse_fallback_text},
    ParseResult,
};

pub fn parse_inline_object(p: HanzzokParser) -> ParseResult<InlineObjectNode> {
    alt((
        map(parse_decorator_chain, InlineObjectNode::DecoratorChain),
        map(parse_escaped_text, InlineObjectNode::Text),
        map(parse_fallback_text, InlineObjectNode::Text),
    ))(p)
}
