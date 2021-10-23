use core::fmt;

pub use block_constructor::{BlockConstructorForm, BlockConstructorNode};
pub use decorator_chain::{DecoratorChainNode, DecoratorNode};
pub use inline_constructor::InlineConstructorNode;
pub use inline_object::InlineObjectNode;
pub use raw::Raw;
pub use text::TextNode;

use crate::syntax::Token;

mod block_constructor;
mod decorator_chain;
mod inline_constructor;
mod inline_object;
mod raw;
mod text;

#[derive(Clone, Debug)]
pub enum HanzzokAstNode {
    BlockConstructor(BlockConstructorNode),
    InlineObjectBlock(Vec<InlineObjectNode>),
    Newline(Vec<Token>),
}

impl Raw for HanzzokAstNode {
    fn raw(&self) -> Vec<Token> {
        match self {
            HanzzokAstNode::BlockConstructor(node) => node.tokens.clone(),
            HanzzokAstNode::InlineObjectBlock(nodes) => nodes.raw(),
            HanzzokAstNode::Newline(tokens) => tokens.clone(),
        }
    }
}
