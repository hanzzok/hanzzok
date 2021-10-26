use crate::{
    codegen::{context::Context, HtmlNode},
    core::ast::InlineObjectNode,
};

use super::Walker;

impl Walker<InlineObjectNode> for Context<'_> {
    fn walk(&mut self, node: InlineObjectNode) -> Vec<HtmlNode> {
        match node {
            InlineObjectNode::DecoratorChain(node) => self.walk(node),
            InlineObjectNode::InlineConstructor(node) => self.walk(node),
            InlineObjectNode::Text(node) => self.walk(node),
        }
    }
}
