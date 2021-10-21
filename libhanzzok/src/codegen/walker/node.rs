use crate::{
    codegen::{context::Context, html::HtmlNode},
    core::ast::HanzzokAstNode,
};

use super::Walker;

impl Walker<HanzzokAstNode> for Context {
    fn walk(&mut self, node: HanzzokAstNode) {
        match node {
            HanzzokAstNode::BlockConstructor(_) => {
                // TODO
            }
            HanzzokAstNode::InlineObject(node) => self.walk(node),
            HanzzokAstNode::Newline(_) => {}
        }
    }
}
