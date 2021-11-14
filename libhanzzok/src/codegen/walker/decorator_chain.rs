use v_htmlescape::escape;

use crate::{
    codegen::{Context, HtmlNode},
    core::ast::{DecoratorChainNode, Raw},
};

use super::Walker;

impl Walker<DecoratorChainNode> for Context<'_> {
    fn walk(&mut self, node: DecoratorChainNode) -> Vec<crate::codegen::HtmlNode> {
        let should_be_raw_until = node
            .decorators
            .iter()
            .enumerate()
            .find(|(_, node)| {
                self.compiler
                    .decorator_rules
                    .get(&node.name)
                    .map(|rule| rule.accept_raw_text())
                    .unwrap_or(false)
            })
            .map(|(index, _)| index + 1)
            .unwrap_or(0);
        let should_be_raw = should_be_raw_until > 0;
        let should_be_raw_until = if should_be_raw {
            should_be_raw_until - 1
        } else {
            should_be_raw_until
        };
        let mut nodes = if should_be_raw {
            vec![HtmlNode::create_text(
                escape(
                    &node
                        .main_text
                        .raw()
                        .into_iter()
                        .chain(
                            node.decorators
                                .iter()
                                .take(should_be_raw_until)
                                .flat_map(|node| node.raw()),
                        )
                        .map(|t| t.text.clone())
                        .collect::<String>(),
                )
                .to_string(),
            )]
        } else {
            self.walk(node.main_text)
        };
        let mut is_first = true;
        for decorator in node.decorators.into_iter().skip(should_be_raw_until) {
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
