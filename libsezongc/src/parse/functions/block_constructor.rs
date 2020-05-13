use super::{text, util::*};
use crate::api::{Ast, BlockConstructor, InlineObject, Span, Spanned, Token, TokenKind};
use crate::parse::{Ignorable, ParseResult, Parser};

pub(crate) fn block_constructor(parser: &mut Parser<'_>) -> ParseResult<Ast> {
    if parser
        .last_used()
        .filter(|token| token.kind == TokenKind::LineWrap)
        .is_none()
    {
        return unmatched(parser);
    }
    let start = exact(TokenKind::PunctuationVerticalLine)(parser)?;
    skip_whitespace(parser);
    let name = exact_require(TokenKind::Text)(parser)?;
    skip_whitespace_with_line_wrap(parser);
    let (input, params, body) = block_constructor_body(parser)?;

    Ok(Ast::BlockConstructor(BlockConstructor {
        name: name.clone(),
        span: Span::from_sequence(
            vec![
                Some(start.span()),
                input.map(|object| object.span()),
                params,
                body,
            ]
            .into_iter()
            .filter_map(|option| option),
        )
        .expect("Same file, more than zero elements"),
    }))
}

fn block_constructor_body(
    parser: &mut Parser<'_>,
) -> ParseResult<(Option<InlineObject>, Option<Span>, Option<Span>)> {
    skip_whitespace(parser);
    let text = text(parser).ignored()?;
    skip_whitespace_with_line_wrap(parser);
    let params = if exact(TokenKind::PunctuationLeftParenthesis)(parser).is_ok() {
        let mut opened = 1;
        Span::from_sequence(
            take_while(exact(move |token: &Token| match token.kind {
                TokenKind::PunctuationLeftParenthesis => {
                    opened += 1;
                    true
                }
                TokenKind::PunctuationRightParenthesis => {
                    opened -= 1;
                    opened != 0
                }
                _ => true,
            }))(parser)?
            .into_iter(),
        )
    } else {
        None
    };
    skip_whitespace_with_line_wrap(parser);
    let body = if let Ok(token) = exact(TokenKind::PunctuationLeftCurlyBracket)(parser) {
        let open_length = token.len();
        Span::from_sequence(
            take_while(exact(move |token: &Token| {
                token.kind == TokenKind::PunctuationRightCurlyBracket && token.len() == open_length
            }))(parser)?
            .into_iter(),
        )
    } else {
        None
    };

    Ok((text, params, body))
}
