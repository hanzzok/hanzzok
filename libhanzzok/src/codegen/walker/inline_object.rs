use crate::{
    codegen::{context::Context, HtmlNode},
    core::ast::InlineObjectNode,
};

use super::Walker;

impl Walker<InlineObjectNode> for Context<'_> {
    fn walk(&self, node: InlineObjectNode) -> Vec<HtmlNode> {
        match node {
            InlineObjectNode::DecoratorChain(node) => self.walk(node),
            InlineObjectNode::InlineConstructor(_) => {
                // TODO
                vec![]
            }
            InlineObjectNode::Text(node) => self.walk(node),
        }
    }
}
