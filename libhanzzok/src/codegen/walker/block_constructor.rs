use crate::{
    codegen::{context::Context, HtmlNode},
    core::ast::BlockConstructorNode,
};

use super::Walker;

impl Walker<BlockConstructorNode> for Context<'_> {
    fn walk(&self, node: BlockConstructorNode) -> Vec<HtmlNode> {
        let rule = match self.compiler.block_constructor_rules.get(&node.name) {
            Some(rule) => rule,
            None => {
                return vec![HtmlNode::create_text(
                    node.tokens
                        .iter()
                        .map(|t| t.text.clone())
                        .collect::<Vec<_>>()
                        .join(""),
                )]
            }
        };
        rule.apply(
            &self.in_container(),
            node.main_text,
            node.param,
            node.multiline_text,
        )
        .into_iter()
        .collect()
    }
}
