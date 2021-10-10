use nom::{
    branch::alt,
    combinator::{map, not, opt},
    multi::{many0, many1},
    sequence::{preceded, terminated, tuple},
};

use crate::{
    core::ast::{DecoratorChainNode, DecoratorNode, InlineObjectNode, TextNode},
    syntax::{Token, TokenKind},
};

use super::{
    nom_ext::{any, satisfy_transform, skip_any_spaces, tag, HanzzokParser},
    parse_inline_constructor::parse_inline_constructor,
    parse_text::{parse_escaped_text, parse_fallback_text},
    ParseResult,
};

fn parse_decorator_params(p: HanzzokParser) -> ParseResult<Vec<Token>> {
    let (p, (l, v, r)) = tuple((
        tag(TokenKind::PunctuationLeftParenthesis),
        many0(alt((
            parse_decorator_params,
            map(
                preceded(
                    not(alt((
                        tag(TokenKind::PunctuationLeftParenthesis),
                        tag(TokenKind::PunctuationRightParenthesis),
                    ))),
                    any,
                ),
                |t| vec![t],
            ),
        ))),
        tag(TokenKind::PunctuationRightParenthesis),
    ))(p)?;

    Ok((p, [vec![l], v.concat(), vec![r]].concat()))
}

pub fn parse_decorator_chain(p: HanzzokParser) -> ParseResult<DecoratorChainNode> {
    let (p, open) = tag(TokenKind::PunctuationLeftSquareBracket)(p)?;

    let (p, _) = skip_any_spaces(p)?;

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

    let (p, _) = skip_any_spaces(p)?;

    let (p, decorators) = many0(terminated(
        map(
            tuple((
                tag(TokenKind::PunctuationFullStop),
                satisfy_transform(|t| match &t.kind {
                    TokenKind::Word(w) => Some(w.clone()),
                    _ => None,
                }),
                opt(parse_decorator_params),
            )),
            |(dot, name, params)| DecoratorNode {
                span: dot
                    .span
                    .joined_opt(params.as_ref().and_then(|params| params.last())),
                name: name.1,
                params: params.map(|params| {
                    let len = params.len();
                    params
                        .into_iter()
                        .take(len - 1)
                        .skip(1)
                        .map(|t| t.text)
                        .collect::<Vec<_>>()
                        .join("")
                }),
            },
        ),
        skip_any_spaces,
    ))(p)?;

    let (p, close) = tag(TokenKind::PunctuationRightSquareBracket)(p)?;

    Ok((
        p,
        DecoratorChainNode {
            main_text: Box::new(main_text),
            span: open.span.joined(&close.span),
            decorators,
        },
    ))
}
