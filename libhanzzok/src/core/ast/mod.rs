use core::fmt;

pub use block_constructor::{BlockConstructorForm, BlockConstructorNode};
pub use decorator_chain::{DecoratorChainNode, DecoratorNode};
pub use inline_constructor::InlineConstructorNode;
pub use inline_object::InlineObjectNode;
pub use text::TextNode;

use super::Span;

mod block_constructor;
mod decorator_chain;
mod inline_constructor;
mod inline_object;
mod text;

#[derive(Clone, Debug)]
pub enum HanzzokAstNode {
    BlockConstructor(BlockConstructorNode),
    InlineObject(InlineObjectNode),
    Newline(Span),
}

impl fmt::Display for HanzzokAstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HanzzokAstNode::BlockConstructor(node) => node.fmt(f),
            HanzzokAstNode::InlineObject(node) => node.fmt(f),
            HanzzokAstNode::Newline(_) => write!(f, "Newline"),
        }
    }
}
