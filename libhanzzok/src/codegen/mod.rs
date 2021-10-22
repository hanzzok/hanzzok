use std::io::{self, Write};

use crate::core::{ast::HanzzokAstNode, Compiler};

pub use self::context::Context;
pub use self::html::HtmlNode;
pub use self::walker::Walker;

mod context;
mod html;
mod walker;

pub fn compile_html(
    nodes: &[HanzzokAstNode],
    compiler: &Compiler,
    w: &mut impl Write,
) -> io::Result<()> {
    let context = Context::new(compiler);

    let html_nodes: Vec<_> = nodes.iter().flat_map(|node| context.walk(node)).collect();
    for html_node in html_nodes {
        html_node.write(w)?;
    }

    Ok(())
}
