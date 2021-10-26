use serde_hzdata::HzdataValue;
use v_htmlescape::escape;

use crate::{
    codegen::{Context, HtmlNode},
    core::ast::InlineConstructorNode,
};

use super::Walker;

impl Walker<InlineConstructorNode> for Context<'_> {
    fn walk(&mut self, node: InlineConstructorNode) -> Vec<crate::codegen::HtmlNode> {
        vec![self
            .compiler
            .inline_constructor_rules
            .get(&node.name)
            .and_then(|rule| {
                let param: String = node.params.iter().map(|t| t.text.clone()).collect();
                let param = serde_hzdata::from_str::<HzdataValue>(&param).ok();

                rule.apply(self, param)
            })
            .unwrap_or_else(|| {
                HtmlNode::create_text(
                    escape(
                        &node
                            .tokens
                            .iter()
                            .map(|t| t.text.clone())
                            .collect::<String>(),
                    )
                    .to_string(),
                )
            })]
    }
}
