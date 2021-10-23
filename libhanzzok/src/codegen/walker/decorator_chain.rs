use crate::{codegen::Context, core::ast::DecoratorChainNode};

use super::Walker;

impl Walker<DecoratorChainNode> for Context<'_> {
    fn walk(&self, node: DecoratorChainNode) -> Vec<crate::codegen::HtmlNode> {
        let mut nodes = self.walk(node.main_text);
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
