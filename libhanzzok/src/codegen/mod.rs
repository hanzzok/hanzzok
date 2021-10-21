use std::io::{self, Write};

use crate::core::ast::HanzzokAstNode;

pub use self::html::HtmlNode;
use self::{context::Context, walker::Walker};

mod context;
mod html;
mod walker;

pub fn compile_html(nodes: &[HanzzokAstNode], w: &mut impl Write) -> io::Result<()> {
    let mut context = Context::new();
    for node in nodes {
        context.walk(node);
    }
    context.write(w)
}
