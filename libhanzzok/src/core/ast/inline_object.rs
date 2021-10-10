use core::fmt;

use super::{DecoratorChainNode, InlineConstructorNode, TextNode};

#[derive(Clone, Debug)]
pub enum InlineObjectNode {
    DecoratorChain(DecoratorChainNode),
    InlineConstructor(InlineConstructorNode),
    Text(TextNode),
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
