use super::{base::ident, text};
use crate::api::parse_prelude::*;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::multispace0,
    multi::many0,
};
use std::sync::{Arc, Mutex};

pub(crate) fn decorator(s: ParserSpan<'_>) -> ParserResult<'_, InlineObject> {
    Spanned::wrap(|s| {
        let (s, _) = tag("[")(s)?;
        let (s, text) = text(s)?;
        let text = text.map(|text| match text {
            InlineObject::PlainText(plain_text_node) => InlineObject::PlainText(PlainText {
                text: plain_text_node.text.trim().to_owned(),
            }),
            _ => text,
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
            InlineObject::Decorator(Decorator {
                text: Box::new(text),
                functions,
            }),
        ))
    })(s)
}

pub(crate) fn decorator_function(s: ParserSpan<'_>) -> ParserResult<'_, DecoratorFunction> {
    Spanned::wrap(|s| {
        let (s, name) = ident(s)?;
        let name = name.value;

        // TODO: waiting for nom 6 and the nom_locate compatible with nom 6
        let opened = Arc::new(Mutex::new(1));
        let (s, params) = match tag::<_, _, ()>("(")(s) {
            Ok((s, _)) => {
                let (s, params) = take_while(move |c| {
                    let mut opened = opened.lock().unwrap();
                    match c {
                        '(' => {
                            *opened += 1;
                            true
                        }
                        ')' => {
                            *opened -= 1;
                            *opened != 0
                        }
                        _ => true,
                    }
                })(s)?;
                let (s, _) = tag(")")(s)?;
                (s, Some(params.fragment().clone().to_owned()))
            }
            _ => (s, None),
        };

        Ok((s, DecoratorFunction { name, params }))
    })(s)
}
