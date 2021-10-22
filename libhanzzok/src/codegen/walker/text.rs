use crate::{
    codegen::{context::Context, html::HtmlNode},
    core::ast::TextNode,
};

use super::Walker;

impl Walker<TextNode> for Context<'_> {
    fn walk(&self, node: TextNode) -> Vec<HtmlNode> {
        let text = HtmlNode::create_text(
            node.tokens
                .iter()
                .filter_map(|(token, show)| {
                    if *show {
                        Some(token.text.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join(""),
        );
        if self.is_in_container {
            vec![text]
        } else {
            vec![HtmlNode::create_tag("p", &[text])]
        }
    }
}
