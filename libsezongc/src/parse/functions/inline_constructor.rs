use super::decorator_function;
use crate::api::parse_prelude::*;
use nom::{bytes::complete::tag, character::complete::multispace0, multi::many0};

pub(crate) fn inline_constructor(s: ParserSpan<'_>) -> ParserResult<'_, InlineObject> {
    Spanned::wrap(|s| {
        let (s, _) = tag("![")(s)?;

        let (s, constructor_function) = decorator_function(s)?;

        let constructor = constructor_function.map(|constructor_function| {
            InlineObject::InlineConstructor(InlineConstructor(Box::new(constructor_function)))
        });

        let (s, functions) = many0(|s| {
            let (s, _) = multispace0(s)?;
            let (s, _) = tag(".")(s)?;
            let (s, _) = multispace0(s)?;
            let (s, function) = decorator_function(s)?;

            Ok((s, function))
        })(s)?;

        let (s, _) = multispace0(s)?;

        let (s, _) = tag("]")(s)?;

        Ok((
            s,
            if functions.is_empty() {
                constructor.value
            } else {
                InlineObject::Decorator(Decorator {
                    text: Box::new(constructor),
                    functions,
                })
            },
        ))
    })(s)
}
