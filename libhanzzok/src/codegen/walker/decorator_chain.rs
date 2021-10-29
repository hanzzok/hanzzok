use v_htmlescape::escape;

use crate::{
    codegen::{Context, HtmlNode},
    core::ast::{DecoratorChainNode, Raw},
};

use super::Walker;

impl Walker<DecoratorChainNode> for Context<'_> {
    fn walk(&mut self, node: DecoratorChainNode) -> Vec<crate::codegen::HtmlNode> {
        let should_be_raw = node
            .decorators
            .first()
            .and_then(|decorator| self.compiler.decorator_rules.get(&decorator.name))
            .map(|rule| rule.accept_raw_text())
            .unwrap_or(false);
        let mut nodes = if should_be_raw {
            vec![HtmlNode::create_text(
                escape(
                    &node
                        .main_text
                        .raw()
                        .iter()
                        .map(|t| t.text.clone())
                        .collect::<String>(),
                )
                .to_string(),
            )]
        } else {
            self.walk(node.main_text)
        };
        let mut is_first = true;
        for decorator in node.decorators {
            let rule = match self.compiler.decorator_rules.get(&decorator.name) {
                Some(rule) => rule,
                None => continue,
            };
            if !is_first && rule.accept_raw_text() {
                continue;
            }
            is_first = false;
            nodes = rule.apply(
                self,
                nodes,
                decorator
                    .params
                    .and_then(|s| serde_hzdata::from_str(&s).ok()),
            );
        }
        nodes
    }
}
