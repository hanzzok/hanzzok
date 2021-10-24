use v_htmlescape::escape;

use crate::{
    codegen::{context::Context, html::HtmlNode},
    core::ast::TextNode,
};

use super::Walker;

impl Walker<TextNode> for Context<'_> {
    fn walk(&self, node: TextNode) -> Vec<HtmlNode> {
        let text = HtmlNode::create_text(
            escape(
                &node
                    .tokens
                    .iter()
                    .filter_map(|(token, show)| {
                        if *show {
                            Some(token.text.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<String>(),
            )
            .to_string(),
        );
        vec![text]
    }
}
