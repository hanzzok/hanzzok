use nom::{
    branch::alt,
    combinator::{fail, map, not, opt},
    multi::many0,
    sequence::{preceded, tuple},
};

use crate::{
    core::{
        ast::{BlockConstructorForm, BlockConstructorNode},
        Spanned,
    },
    syntax::{parse::nom_ext::any, Token, TokenKind},
};

use super::{
    nom_ext::{satisfy_transform, skip_horizontal_spaces, tag, HanzzokParser},
    parse_inline_object::parse_inline_object,
    Error, ParseResult,
};

pub fn parse_block_constructor(p: HanzzokParser) -> ParseResult<BlockConstructorNode> {
    alt((
        parse_block_constructor_basic,
        parse_block_constructor_shortened,
    ))(p)
}

fn parse_block_constructor_params(p: HanzzokParser) -> ParseResult<Vec<Token>> {
    let (p, (l, v, r)) = tuple((
        tag(TokenKind::PunctuationLeftCurlyBracket),
        many0(alt((
            parse_block_constructor_params,
            map(
                preceded(
                    not(alt((
                        tag(TokenKind::PunctuationLeftCurlyBracket),
                        tag(TokenKind::PunctuationRightCurlyBracket),
                    ))),
                    any,
                ),
                |t| vec![t],
            ),
        ))),
        tag(TokenKind::PunctuationRightCurlyBracket),
    ))(p)?;

    Ok((p, [vec![l], v.concat(), vec![r]].concat()))
}

pub fn parse_block_constructor_basic(p: HanzzokParser) -> ParseResult<BlockConstructorNode> {
    let (p, _) = tag(TokenKind::VerticalSpace)(p)?;
    let (p, vertical_bar) = tag(TokenKind::PunctuationVerticalBar)(p)?;

    let (p, _) = skip_horizontal_spaces(p)?;

    let (p, name) = satisfy_transform(|t| match &t.kind {
        TokenKind::Word(w) => Some(w.clone()),
        _ => None,
    })(p)?;

    let (p, _) = skip_horizontal_spaces(p)?;

    let (p, main_text) = many0(preceded(
        not(alt((
            tag(TokenKind::PunctuationLeftCurlyBracket),
            tag(TokenKind::VerticalSpace),
        ))),
        parse_inline_object,
    ))(p)?;

    let (p, param) = opt(map(parse_block_constructor_params, |params| {
        let collected: Vec<_> = params.iter().map(|t| t.text.as_ref()).collect::<Vec<_>>();
        (
            collected.clone().join(""),
            params[0].span.joined_opt(params.last()),
        )
    }))(p)?;
    let param = param.as_ref();

    let multiline_text = Vec::new();

    let maintext_last_span = main_text.last().map(|node| node.span());
    let last_span = param
        .map(|(_, s)| s)
        .or(maintext_last_span.as_ref())
        .unwrap_or(&name.0.span);

    Ok((
        p,
        BlockConstructorNode {
            form: BlockConstructorForm::Basic,
            name: name.1,
            main_text,
            param: param.map(|(s, _)| s.clone()),
            multiline_text,
            span: vertical_bar.span.joined(last_span),
        },
    ))
}

pub fn parse_block_constructor_shortened(p: HanzzokParser) -> ParseResult<BlockConstructorNode> {
    let (p, _) = tag(TokenKind::VerticalSpace)(p)?;

    let (p, name) = match p
        .block_constructors
        .get(&BlockConstructorForm::Shortened)
        .iter()
        .flat_map(|v| v.iter())
        .find_map(|parser| parser.parse(p.clone()).ok())
    {
        Some(v) => v,
        None => return fail(p),
    };
    let name_span = name[0].span.joined_opt(name.last());
    let name: String = name.iter().map(|t| t.text.clone()).collect();

    let (p, _) = skip_horizontal_spaces(p)?;

    let (p, main_text) = many0(preceded(
        not(alt((
            tag(TokenKind::PunctuationLeftCurlyBracket),
            tag(TokenKind::VerticalSpace),
        ))),
        parse_inline_object,
    ))(p)?;

    let (p, param) = opt(map(parse_block_constructor_params, |params| {
        let collected: Vec<_> = params.iter().map(|t| t.text.as_ref()).collect::<Vec<_>>();
        (
            collected.clone().join(""),
            params[0].span.joined_opt(params.last()),
        )
    }))(p)?;
    let param = param.as_ref();

    let multiline_text = Vec::new();

    let maintext_last_span = main_text.last().map(|node| node.span());
    let last_span = param
        .map(|(_, s)| s)
        .or(maintext_last_span.as_ref())
        .unwrap_or(&name_span);

    Ok((
        p,
        BlockConstructorNode {
            form: BlockConstructorForm::Shortened,
            name,
            main_text,
            param: param.map(|(s, _)| s.clone()),
            multiline_text,
            span: name_span.joined(last_span),
        },
    ))
}
