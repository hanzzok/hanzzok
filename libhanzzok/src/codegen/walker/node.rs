use crate::{
    codegen::{context::Context, HtmlNode},
    core::ast::HanzzokAstNode,
};

use super::Walker;

impl Walker<HanzzokAstNode> for Context<'_> {
    fn walk(&self, node: HanzzokAstNode) -> Vec<HtmlNode> {
        match node {
            HanzzokAstNode::BlockConstructor(node) => self.walk(node),
            HanzzokAstNode::InlineObject(node) => self.walk(node),
            HanzzokAstNode::Newline(_) => {
                // Should not add something.
                vec![]
            }
        }
    }
}
