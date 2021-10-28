use std::io::{self, Write};

use crate::core::{ast::HanzzokAstNode, Compiler};

pub use self::context::Context;
pub use self::html::{HtmlNode, HtmlNodeRef};
pub use self::walker::Walker;

mod context;
mod html;
mod walker;

pub fn compile_html(
    nodes: &[HanzzokAstNode],
    compiler: &Compiler,
    w: &mut impl Write,
) -> io::Result<()> {
    let mut context = Context::new(compiler);

    let html_nodes: Vec<_> = nodes.iter().flat_map(|node| context.walk(node)).collect();
    for html_node in html_nodes {
        html_node.write(&context, w)?;
    }

    Ok(())
}

pub fn compile_html_with_hint<'a>(
    nodes: &'a [HanzzokAstNode],
    compiler: &'a Compiler,
) -> (Context<'a>, Vec<(HanzzokAstNode, Vec<HtmlNode>)>) {
    let mut context = Context::new(compiler);

    let html_nodes: Vec<_> = nodes
        .iter()
        .map(|node| (node.clone(), context.walk(node)))
        .collect();

    (context, html_nodes)
}
