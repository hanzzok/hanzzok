use core::fmt;

use crate::{
    core::{Span, Spanned},
    syntax::Token,
};

use super::Raw;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct TextNode {
    pub(crate) tokens: Vec<(Token, bool)>,
}

impl TextNode {
    pub fn merged_with(&self, other: &TextNode) -> TextNode {
        TextNode {
            tokens: [self.tokens.clone(), other.tokens.clone()].concat(),
        }
    }
}

impl Raw for TextNode {
    fn raw(&self) -> Vec<Token> {
        self.tokens.iter().map(|(token, _)| token.clone()).collect()
    }
}

impl fmt::Display for TextNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text[")?;
        let mut has_written = false;
        for (token, show) in &self.tokens {
            if !show {
                continue;
            }
            if has_written {
                write!(f, ", ")?;
            }
            token.fmt(f)?;
            has_written = true;
        }
        write!(f, "]")
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
