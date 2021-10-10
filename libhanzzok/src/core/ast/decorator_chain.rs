use core::fmt;

use crate::core::Span;

use super::InlineObjectNode;

#[derive(Clone, Debug)]
pub struct DecoratorChainNode {
    pub span: Span,
    pub main_text: Box<InlineObjectNode>,
}

impl fmt::Display for DecoratorChainNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DecoratorChain(main_text={})", self.main_text)
    }
}

#[derive(Clone, Debug)]
pub struct Decorator {
    span: Span,
    name: String,
}
