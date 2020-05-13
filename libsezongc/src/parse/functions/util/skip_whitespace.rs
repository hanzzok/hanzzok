use super::*;
use crate::api::{Token, TokenKind};
use crate::parse::Parser;

pub(crate) fn skip_whitespace(parser: &mut Parser<'_>) {
    take_while(exact(TokenKind::Whitespace))(parser).expect("Must be no error");
}

pub(crate) fn skip_whitespace_with_line_wrap(parser: &mut Parser<'_>) {
    let mut line_wrap = false;
    take_while(exact(|token: &Token| match token.kind {
        TokenKind::Whitespace => true,
        TokenKind::LineWrap => {
            if line_wrap {
                false
            } else {
                line_wrap = true;
                true
            }
        }
        _ => false,
    }))(parser)
    .expect("Must be no error");
}
