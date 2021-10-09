use core::fmt;

use crate::{
    core::{DisplayWithoutSpan, Span, Spanned},
    syntax::Token,
};

#[derive(Clone, Debug)]
pub struct TextNode {
    pub tokens: Vec<(Token, bool)>,
}

impl fmt::Display for TextNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text(")?;
        let mut has_written = false;
        for (token, _) in &self.tokens {
            if has_written {
                write!(f, ", ")?;
            }
            token.fmt_without_span(f)?;
            has_written = true
        }
        write!(f, ") at {}", self.span())
    }
}

impl Spanned for TextNode {
    fn span(&self) -> Span {
        self.tokens[0]
            .0
            .span
            .joined_opt(self.tokens.last().map(|(token, _)| token))
    }
}
