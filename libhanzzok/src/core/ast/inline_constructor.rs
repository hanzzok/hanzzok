use core::fmt;

use crate::{
    core::{Span, Spanned},
    syntax::Token,
};

use super::Raw;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct InlineConstructorNode {
    pub(crate) name: String,
    pub(crate) params: Vec<Token>,
    pub(crate) tokens: Vec<Token>,
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
