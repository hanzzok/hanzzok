use core::fmt;

pub use decorator_chain::{DecoratorChainNode, DecoratorNode};
pub use inline_constructor::InlineConstructorNode;
pub use inline_object::InlineObjectNode;
pub use text::TextNode;

mod decorator_chain;
mod inline_constructor;
mod inline_object;
mod text;

#[derive(Clone, Debug)]
pub enum HanzzokAstNode {
    InlineObject(InlineObjectNode),
}

impl fmt::Display for HanzzokAstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HanzzokAstNode::InlineObject(node) => node.fmt(f),
        }
    }
}
