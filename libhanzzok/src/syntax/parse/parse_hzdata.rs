use std::iter::once;

use nom::{
    branch::alt,
    combinator::{map, not},
    multi::many0,
    sequence::{preceded, tuple},
};

use crate::syntax::{Token, TokenKind};

use super::{
    nom_ext::{any, tag, HanzzokParser},
    ParseResult,
};

pub fn parse_hzdata_string(p: HanzzokParser) -> ParseResult<Vec<Token>> {
    map(
        tuple((
            tag(TokenKind::PunctuationQuotationMark),
            many0(alt((
                map(
                    preceded(not(tag(TokenKind::PunctuationQuotationMark)), any),
                    |t| vec![t],
                ),
                map(
                    tuple((
                        tag(TokenKind::PunctuationReverseSolidus),
                        tag(TokenKind::PunctuationQuotationMark),
                    )),
                    |(a, b)| vec![a, b],
                ),
            ))),
            tag(TokenKind::PunctuationQuotationMark),
        )),
        |(q1, v, q2)| {
            once(q1)
                .chain(v.into_iter().flatten())
                .chain(once(q2))
                .collect()
        },
    )(p)
}

pub fn parse_hzdata_paired(
    left: TokenKind,
    right: TokenKind,
    exclude_outer: bool,
) -> impl Fn(HanzzokParser) -> ParseResult<Vec<Token>> {
    move |p| {
        let (p, (l, v, r)) = tuple((
            tag(left.clone()),
            many0(alt((
                parse_hzdata_string,
                parse_hzdata_paired(left.clone(), right.clone(), false),
                map(
                    preceded(not(alt((tag(left.clone()), tag(right.clone())))), any),
                    |t| vec![t],
                ),
            ))),
            tag(right.clone()),
        ))(p)?;

        if exclude_outer {
            Ok((p, v.concat()))
        } else {
            Ok((p, [vec![l], v.concat(), vec![r]].concat()))
        }
    }
}
