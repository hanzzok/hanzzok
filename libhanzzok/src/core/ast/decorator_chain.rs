use core::fmt;

use crate::{
    core::{Span, Spanned},
    syntax::Token,
};

use super::{InlineObjectNode, Raw};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct DecoratorChainNode {
    pub(crate) main_text: Box<InlineObjectNode>,
    pub(crate) decorators: Vec<DecoratorNode>,
    pub(crate) tokens: Vec<Token>,
}

impl Spanned for DecoratorChainNode {
    fn span(&self) -> Span {
        self.tokens.span()
    }
}

impl Raw for DecoratorChainNode {
    fn raw(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

impl fmt::Display for DecoratorChainNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DecoratorChain(main_text={}, decorators=[",
            self.main_text
        )?;
        let mut has_written = false;
        for decorator in &self.decorators {
            if has_written {
                write!(f, ", ")?;
            }
            decorator.fmt(f)?;
            has_written = true;
        }

        write!(f, "])")
    }
}

#[derive(Clone, Debug)]
pub struct DecoratorNode {
    pub span: Span,
    pub name: String,
    pub params: Option<String>,
}

impl fmt::Display for DecoratorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Decorator(name={}{})",
            self.name,
            match &self.params {
                Some(v) => format!(", params={:?}", v),
                None => "".to_owned(),
            }
        )
    }
}
