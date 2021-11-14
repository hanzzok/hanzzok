use nom::{
    branch::alt,
    combinator::{map, not, opt},
    multi::{many0, many1},
    sequence::{preceded, terminated},
};

use crate::{
    core::ast::{DecoratorChainNode, DecoratorNode, InlineObjectNode, TextNode},
    syntax::{Token, TokenKind},
};

use super::{
    nom_ext::{satisfy_transform, skip_any_spaces, tag, HanzzokParser},
    parse_hzdata::parse_hzdata_paired,
    parse_inline_constructor::parse_inline_constructor,
    parse_text::{parse_escaped_text, parse_fallback_text},
    ParseResult,
};

fn parse_decorator_params(p: HanzzokParser) -> ParseResult<Vec<Token>> {
    parse_hzdata_paired(
        TokenKind::PunctuationLeftParenthesis,
        TokenKind::PunctuationRightParenthesis,
        true,
    )(p)
}

pub fn parse_decorator_chain(p: HanzzokParser) -> ParseResult<DecoratorChainNode> {
    let tt = p.create_tracker();

    let (p, _) = tag(TokenKind::PunctuationLeftSquareBracket)(p)?;

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
                    tokens: {
                        let mut tokens: Vec<_> = nodes
                            .into_iter()
                            .flat_map(|node| node.tokens.into_iter())
                            .collect();

                        while let Some((
                            (
                                Token {
                                    kind: TokenKind::HorizontalSpace,
                                    ..
                                },
                                _,
                            ),
                            elements,
                        )) = tokens.split_last_mut()
                        {
                            if let Some((
                                Token {
                                    kind: TokenKind::PunctuationReverseSolidus,
                                    ..
                                },
                                _,
                            )) = elements.last()
                            {
                                break;
                            }

                            tokens = elements.to_vec();
                        }

                        tokens
                    },
                })
            },
        ),
    ))(p)?;

    let (p, _) = skip_any_spaces(p)?;

    let (p, decorators) = many0(terminated(
        |p: HanzzokParser| {
            let tt = p.create_tracker();
            let (p, _) = tag(TokenKind::PunctuationFullStop)(p)?;
            let (p, name) = many1(satisfy_transform(|t| match &t.kind {
                TokenKind::Word(w) => Some(w.clone()),
                TokenKind::PunctuationHyphenMinus => Some("-".to_owned()),
                _ => None,
            }))(p)?;
            let (p, params) = opt(parse_decorator_params)(p)?;
            let tokens = tt.end(&p);
            Ok((
                p,
                DecoratorNode {
                    tokens,
                    name: name.into_iter().map(|(_, name)| name).collect(),
                    params: params
                        .map(|params| params.into_iter().map(|t| t.text).collect::<String>()),
                },
            ))
        },
        skip_any_spaces,
    ))(p)?;

    let (p, _) = tag(TokenKind::PunctuationRightSquareBracket)(p)?;

    let tokens = tt.end(&p);

    Ok((
        p,
        DecoratorChainNode {
            main_text: Box::new(main_text),
            decorators,
            tokens,
        },
    ))
}
