use std::iter::once;

use nom::{
    branch::alt,
    combinator::{fail, map, not, opt},
    multi::many0,
    sequence::{preceded, tuple},
    InputIter,
};

use crate::{
    core::ast::{
        BlockConstructorForm, BlockConstructorNode, InlineConstructorNode, InlineObjectNode,
        TextNode,
    },
    syntax::{
        parse::{
            nom_ext::{any, skip_vertical_spaces},
            parse_text::parse_single_newline,
        },
        Token, TokenKind,
    },
};

use super::{
    nom_ext::{
        satisfy_transform, skip_any_spaces, skip_horizontal_spaces, tag,
        BlockConstructorNameParser, HanzzokParser,
    },
    parse_inline_object::parse_inline_object,
    ParseResult,
};

pub fn parse_block_constructor(p: HanzzokParser) -> ParseResult<BlockConstructorNode> {
    alt((
        parse_block_constructor_basic,
        parse_block_constructor_leading,
        parse_block_constructor_bookend,
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
    let tt = p.create_tracker();

    let (p, _) = tag(TokenKind::PunctuationVerticalBar)(p)?;

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
        let collected: Vec<_> = params.iter().map(|t| t.text.as_ref()).collect();
        (
            collected.clone().join(""),
            params[0].span.joined_opt(params.last()),
        )
    }))(p)?;
    let param = param.as_ref();

    let tokens = tt.end(&p);

    Ok((
        p,
        BlockConstructorNode {
            form: BlockConstructorForm::Basic,
            name: name.1,
            main_text,
            param: param.map(|(s, _)| s.clone()),
            multiline_text: Vec::new(),
            tokens,
        },
    ))
}

pub fn parse_block_constructor_shortened(p: HanzzokParser) -> ParseResult<BlockConstructorNode> {
    let tt = p.create_tracker();

    let (p, name) = match p
        .block_constructors
        .get(&BlockConstructorForm::Shortened)
        .into_iter()
        .flatten()
        .filter_map(|(parser, _)| parser.parse(p.clone()).ok())
        .max_by_key(|(_, t)| t.len())
    {
        Some(v) => v,
        None => return fail(p),
    };
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
        let collected: Vec<_> = params.iter().map(|t| t.text.as_ref()).collect();
        (
            collected.clone().join(""),
            params[0].span.joined_opt(params.last()),
        )
    }))(p)?;
    let param = param.as_ref();

    let tokens = tt.end(&p);

    Ok((
        p,
        BlockConstructorNode {
            form: BlockConstructorForm::Shortened,
            name,
            main_text,
            param: param.map(|(s, _)| s.clone()),
            multiline_text: Vec::new(),
            tokens,
        },
    ))
}

fn parse_block_constructor_leading(p: HanzzokParser) -> ParseResult<BlockConstructorNode> {
    let tt = p.create_tracker();

    let (parser, _) = match p
        .block_constructors
        .get(&BlockConstructorForm::Leading)
        .into_iter()
        .flatten()
        .filter_map(|(parser, _)| parser.parse(p.clone()).ok().map(|(_, t)| (parser, t)))
        .max_by_key(|(_, t)| t.len())
    {
        Some(v) => v,
        None => return fail(p),
    };
    let parser = parser.clone();

    let (p, multiline_text) = parse_block_constructor_leading_base(&parser, p)?;

    let tokens = tt.end(&p);

    Ok((
        p,
        BlockConstructorNode {
            form: BlockConstructorForm::Leading,
            name: parser.name,
            main_text: Vec::new(),
            param: None,
            multiline_text,
            tokens,
        },
    ))
}

fn parse_block_constructor_leading_base(
    name_parser: &BlockConstructorNameParser,
    p: HanzzokParser,
) -> ParseResult<Vec<Vec<InlineObjectNode>>> {
    let (p, _) = name_parser.parse(p)?;

    let (p, _) = skip_horizontal_spaces(p)?;

    let (p, main_text) = many0(preceded(
        not(tag(TokenKind::VerticalSpace)),
        parse_inline_object,
    ))(p)?;

    let (p, _) = skip_any_spaces(p)?;

    let (p, next) = opt(|p| parse_block_constructor_leading_base(name_parser, p))(p)?;

    Ok((
        p,
        once(main_text).chain(next.into_iter().flatten()).collect(),
    ))
}

fn parse_block_constructor_bookend(p: HanzzokParser) -> ParseResult<BlockConstructorNode> {
    let tt = p.create_tracker();

    let (parser, block_constructor, _) = match p
        .block_constructors
        .get(&BlockConstructorForm::Bookend)
        .into_iter()
        .flatten()
        .filter_map(|(parser, block_constructor)| {
            parser
                .parse(p.clone())
                .ok()
                .map(|(_, t)| (parser, block_constructor, t))
        })
        .max_by_key(|(_, _, t)| t.len())
    {
        Some(v) => v,
        None => return fail(p),
    };
    let accept_raw_multiline = block_constructor.accept_raw_multiline();
    let parser = parser.clone();

    let (p, _) = parser.parse(p)?;

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

    let (p, _) = skip_vertical_spaces(p)?;

    let (p, multiline_text): (HanzzokParser, Vec<InlineObjectNode>) = if accept_raw_multiline {
        let (p, tokens) = many0(preceded(
            not(|p| parser.parse(p)),
            alt((
                map(
                    tuple((tag(TokenKind::PunctuationReverseSolidus), |p| {
                        parser.parse(p)
                    })),
                    |(l, r)| [vec![l], r].concat(),
                ),
                map(any, |t| vec![t]),
            )),
        ))(p)?;

        (
            p,
            vec![InlineObjectNode::Text(TextNode {
                tokens: tokens.iter().flatten().map(|t| (t.clone(), true)).collect(),
            })],
        )
    } else {
        many0(preceded(
            not(|p| parser.parse(p)),
            alt((
                parse_inline_object,
                map(parse_single_newline, InlineObjectNode::Text),
            )),
        ))(p)?
    };

    let (p, _) = skip_any_spaces(p)?;

    let (p, _) = parser.parse(p)?;

    let tokens = tt.end(&p);

    Ok((
        p,
        BlockConstructorNode {
            form: BlockConstructorForm::Bookend,
            name: parser.name,
            main_text,
            param: param.map(|(s, _)| s.clone()),
            multiline_text: vec![multiline_text],
            tokens,
        },
    ))
}
