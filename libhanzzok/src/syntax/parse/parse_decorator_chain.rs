use nom::{
    branch::alt,
    bytes::streaming::take_until1,
    combinator::{cond, map, map_parser, not, recognize},
    multi::many1,
    sequence::preceded,
};

use crate::{
    core::ast::{DecoratorChainNode, InlineObjectNode, TextNode},
    syntax::TokenKind,
};

use super::{
    nom_ext::{tag, HanzzokParser},
    parse_inline_constructor::parse_inline_constructor,
    parse_text::{parse_escaped_text, parse_fallback_text},
    ParseResult,
};

pub fn parse_decorator_chain(p: HanzzokParser) -> ParseResult<DecoratorChainNode> {
    let (p, open) = tag(TokenKind::PunctuationLeftSquareBracket)(p)?;

    let (p, main_text) = alt((
        map(
            parse_inline_constructor,
            InlineObjectNode::InlineConstructor,
        ),
        map(
            many1(alt((
                parse_escaped_text,
                preceded(
                    not(alt((
                        tag(TokenKind::PunctuationFullStop),
                        tag(TokenKind::PunctuationRightSquareBracket),
                    ))),
                    parse_fallback_text,
                ),
            ))),
            |nodes| {
                InlineObjectNode::Text(TextNode {
                    tokens: nodes
                        .into_iter()
                        .flat_map(|node| node.tokens.into_iter())
                        .collect(),
                })
            },
        ),
    ))(p)?;

    let (p, close) = tag(TokenKind::PunctuationRightSquareBracket)(p)?;

    Ok((
        p,
        DecoratorChainNode {
            main_text: Box::new(main_text),
            span: open.span.joined(&close.span),
        },
    ))
}
