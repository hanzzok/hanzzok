pub use block_constructor::{BlockConstructorForm, BlockConstructorNode};
pub use decorator_chain::{DecoratorChainNode, DecoratorNode};
pub use inline_constructor::InlineConstructorNode;
pub use inline_object::InlineObjectNode;
pub use raw::Raw;
pub use text::TextNode;
#[cfg(target_arch = "wasm32")]
pub use wasm_support::HanzzokAstNodeWrapper;

use crate::syntax::Token;

mod block_constructor;
mod decorator_chain;
mod inline_constructor;
mod inline_object;
mod raw;
mod text;

#[cfg(target_arch = "wasm32")]
mod wasm_support;

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
