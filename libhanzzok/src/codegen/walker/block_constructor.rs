use crate::{
    codegen::{context::Context, HtmlNode},
    core::ast::BlockConstructorNode,
};

use super::Walker;

impl Walker<BlockConstructorNode> for Context<'_> {
    fn walk(&mut self, node: BlockConstructorNode) -> Vec<HtmlNode> {
        let rule = match self.compiler.block_constructor_rules.get(&node.name) {
            Some(rule) => rule,
            None => {
                return vec![HtmlNode::create_text(
                    node.tokens
                        .iter()
                        .map(|t| t.text.clone())
                        .collect::<String>(),
                )];
            }
        };
        vec![rule.apply(
            self,
            node.main_text,
            node.param.and_then(|s| serde_hzdata::from_str(&s).ok()),
            node.multiline_text,
        )]
    }
}
