use crate::{
    codegen::{context::Context, html::HtmlNode},
    core::ast::TextNode,
};

use super::Walker;

impl Walker<TextNode> for Context {
    fn walk(&mut self, node: TextNode) {
        self.push(HtmlNode::create_tag(
            "p",
            &[HtmlNode::create_text(
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
            )],
        ))
    }
}
