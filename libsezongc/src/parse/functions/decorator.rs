use super::{fallback_text, text, util::*};
use crate::api::{
    Ast, Decorator, DecoratorFunction, InlineObject, PlainText, Span, Spanned, Token, TokenKind,
};
use crate::parse::{ParseResult, Parser};

pub(crate) fn decorator(parser: &mut Parser<'_>) -> ParseResult<InlineObject> {
    let start = exact(TokenKind::PunctuationLeftSquareBracket)(parser)?;
    let text = take_while(any((text, |parser: &mut Parser<'_>| {
        match parser.tokens[parser.cursor].kind {
            TokenKind::PunctuationFullStop | TokenKind::PunctuationRightSquareBracket => {
                unmatched(parser)
            }
            _ => fallback_text(parser),
        }
    })))(parser)?;

    if text.is_empty() {
        return Ok(InlineObject::PlainText(PlainText { span: start.span() }));
    }

    let text = InlineObject::PlainText(PlainText {
        span: Span::from_sequence(text.into_iter()).expect("Same file, not empty"),
    });

    let functions = take_while(|parser| {
        skip_whitespace_with_line_wrap(parser);
        let dot = exact(TokenKind::PunctuationFullStop)(parser)?;
        skip_whitespace_with_line_wrap(parser);
        let function = decorator_function(parser)?;

        Ok(DecoratorFunction {
            span: dot.span().joined(&function.span).expect("Same file"),
            ..function
        })
    })(parser)?;

    skip_whitespace_with_line_wrap(parser);

    let end = exact_require(TokenKind::PunctuationRightSquareBracket)(parser)?;

    Ok(InlineObject::Decorator(Decorator {
        text: Box::new(text),
        functions,
        span: start.span().joined(&end.span()).expect("Same file"),
    }))
}

pub(crate) fn decorator_function(parser: &mut Parser<'_>) -> ParseResult<DecoratorFunction> {
    let name = exact_require(TokenKind::Text)(parser)?;

    let mut opened = 1;
    let params = if exact(TokenKind::PunctuationLeftParenthesis)(parser).is_ok() {
        Span::from_sequence(
            take_while(exact(move |token: &Token| match token.kind {
                _ if opened == 0 => false,
                TokenKind::PunctuationLeftParenthesis => {
                    opened += 1;
                    true
                }
                TokenKind::PunctuationRightParenthesis => {
                    opened -= 1;
                    true
                }
                _ => true,
            }))(parser)?
            .into_iter(),
        )
    } else {
        None
    };

    let whole_span = if let Some(params) = &params {
        name.span().joined(params).expect("Same file")
    } else {
        name.span()
    };

    Ok(DecoratorFunction {
        name,
        params,
        span: whole_span,
    })
}
