use crate::{
    codegen::{context::Context, html::HtmlNode},
    core::ast::InlineObjectNode,
};

use super::Walker;

impl Walker<InlineObjectNode> for Context {
    fn walk(&mut self, node: InlineObjectNode) {
        match node {
            InlineObjectNode::DecoratorChain(_) => {
                // TODO
            }
            InlineObjectNode::InlineConstructor(_) => {
                // TODO
            }
            InlineObjectNode::Text(node) => self.walk(node),
        }
    }
}
