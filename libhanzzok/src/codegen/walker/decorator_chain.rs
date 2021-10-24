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
        for decorator in node.decorators {
            let rule = match self.compiler.decorator_rules.get(&decorator.name) {
                Some(rule) => rule,
                None => continue,
            };
            nodes = rule.apply(self, nodes, decorator.params);
        }
        nodes
    }
}
