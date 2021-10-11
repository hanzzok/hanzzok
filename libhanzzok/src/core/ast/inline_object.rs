use core::fmt;

use crate::core::{Span, Spanned};

use super::{DecoratorChainNode, InlineConstructorNode, TextNode};

#[derive(Clone, Debug)]
pub enum InlineObjectNode {
    DecoratorChain(DecoratorChainNode),
    InlineConstructor(InlineConstructorNode),
    Text(TextNode),
}

impl Spanned for InlineObjectNode {
    fn span(&self) -> Span {
        match self {
            InlineObjectNode::DecoratorChain(node) => node.span(),
            InlineObjectNode::InlineConstructor(node) => node.span(),
            InlineObjectNode::Text(node) => node.span(),
        }
    }
}

impl fmt::Display for InlineObjectNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InlineObjectNode::DecoratorChain(node) => node.fmt(f),
            InlineObjectNode::InlineConstructor(node) => node.fmt(f),
            InlineObjectNode::Text(node) => node.fmt(f),
        }
    }
}
