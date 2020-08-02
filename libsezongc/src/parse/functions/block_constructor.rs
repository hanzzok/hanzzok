use super::{
    base::{ident, multispace, tag},
    text,
};
use crate::api::parse_prelude::*;
use nom::{
    branch::alt,
    bytes::complete::{take_until, take_while},
};
use std::sync::{Arc, Mutex};

pub(crate) fn block_constructor(s: ParserSpan<'_>) -> ParserResult<'_, Ast> {
    let (s, block_constructor) = Spanned::wrap(|s| {
        let (s, _) = alt((tag("\r\n"), tag("\n"), tag("\r")))(s).or_else(|err| {
            if s.location_offset() == 0 {
                Ok((s, s))
            } else {
                Err(err)
            }
        })?;
        let (s, _) = tag("|")(s)?;
        let s = multispace(s);
        let (s, name) = ident(s)?;
        let (s, (input, params, body)) =
            block_constructor_body(s).unwrap_or_else(|_| (s, (None, None, None)));

        Ok((
            s,
            BlockConstructor {
                name,
                input,
                params,
                body,
            },
        ))
    })(s)?;

    Ok((
        s,
        block_constructor.map_wrapped(|block_constructor| Ast::BlockConstructor(block_constructor)),
    ))
}

fn block_constructor_body(
    s: ParserSpan<'_>,
) -> ParserResultBase<
    '_,
    (
        Option<Spanned<InlineObject>>,
        Option<String>,
        Option<String>,
    ),
> {
    let s = multispace(s);
    let (s, text) = text(s)
        .ok()
        .map(|(s, text)| (s, Some(text)))
        .unwrap_or_else(|| (s, None));

    let s = multispace(s);

    let (s, params) = match tag("(")(s) {
        Ok((s, _)) => {
            // TODO: waiting for nom 6 and the nom_locate compatible with nom 6
            let opened = Arc::new(Mutex::new(1));
            let (s, params) = take_while(|c| {
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
            (s, Some(params.clone().to_owned()))
        }
        _ => (s, None),
    };
    let params = params.map(|params| params.fragment().clone().to_owned());

    let s = multispace(s);

    let (s, body) = match tag("{")(s) {
        Ok((s, token)) => {
            let open_length = token.fragment().len();
            let (s, body) = take_until("}".repeat(open_length).as_ref())(s)?;
            (s, Some(body.clone().to_owned()))
        }
        _ => (s, None),
    };
    let body = body.map(|body| body.fragment().clone().to_owned());

    Ok((s, (text, params, body)))
}
