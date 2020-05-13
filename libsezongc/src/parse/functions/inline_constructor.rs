use super::{decorator_function, util::*};
use crate::api::{
    Decorator, DecoratorFunction, InlineConstructor, InlineObject, Spanned, TokenKind,
};
use crate::parse::{ParseResult, Parser};

pub(crate) fn inline_constructor(parser: &mut Parser<'_>) -> ParseResult<InlineObject> {
    let start = exact(TokenKind::PunctuationExclamationMark)(parser)?;
    exact(TokenKind::PunctuationLeftSquareBracket)(parser)?;
    let constructor_function = decorator_function(parser)?;

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

    let constructor = InlineObject::InlineConstructor(InlineConstructor {
        constructor_function: Box::new(constructor_function),
        span: start.span().joined(&end.span()).expect("Same file"),
    });

    Ok(if functions.is_empty() {
        constructor
    } else {
        InlineObject::Decorator(Decorator {
            text: Box::new(constructor),
            functions,
            span: start.span().joined(&end.span()).expect("Same file"),
        })
    })
}
