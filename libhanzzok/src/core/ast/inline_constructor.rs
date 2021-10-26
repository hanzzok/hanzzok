use core::fmt;

use crate::{
    core::{Span, Spanned},
    syntax::Token,
};

use super::Raw;

#[derive(Clone, Debug)]
pub struct InlineConstructorNode {
    pub name: String,
    pub params: Vec<Token>,
    pub tokens: Vec<Token>,
}

impl Raw for InlineConstructorNode {
    fn raw(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

impl Spanned for InlineConstructorNode {
    fn span(&self) -> Span {
        self.tokens.span()
    }
}

impl fmt::Display for InlineConstructorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InlineConstructor(name={})", self.name)
    }
}
