use core::fmt;

use crate::core::{Span, Spanned};

#[derive(Clone, Debug)]
pub struct InlineConstructorNode {
    pub span: Span,
    pub name: String,
}

impl Spanned for InlineConstructorNode {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl fmt::Display for InlineConstructorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InlineConstructor(name={})", self.name)
    }
}
