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

pub fn compile_html_nodes<'a>(
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

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn w_compile_html(
    nodes: &crate::syntax::HanzzokParsed,
    compiler: &Compiler,
) -> Result<String, wasm_bindgen::JsValue> {
    let mut cursor = io::Cursor::new(Vec::new());
    match compile_html(
        &nodes
            .nodes
            .iter()
            .map(|node| node.handle.clone())
            .collect::<Vec<_>>(),
        compiler,
        &mut cursor,
    ) {
        Ok(_) => Ok(String::from_utf8_lossy(&cursor.into_inner()).to_string()),
        Err(e) => Err(wasm_bindgen::JsValue::from_str(&e.to_string())),
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub struct HanzzokCompiled {
    htmls: Vec<String>,
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
impl HanzzokCompiled {
    pub fn next(&mut self) -> Option<String> {
        if self.htmls.len() > 0 {
            Some(self.htmls.remove(0))
        } else {
            None
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn w_compile_html_nodes(
    nodes: &crate::syntax::HanzzokParsed,
    compiler: &Compiler,
) -> Result<HanzzokCompiled, wasm_bindgen::JsValue> {
    let nodes: Vec<_> = nodes.nodes.iter().map(|node| node.handle.clone()).collect();
    let (context, nodes) = compile_html_nodes(&nodes, compiler);
    Ok(HanzzokCompiled {
        htmls: nodes
            .into_iter()
            .map(|(_, nodes)| {
                let mut cursor = io::Cursor::new(Vec::new());
                for node in nodes {
                    if let Err(e) = node.write(&context, &mut cursor) {
                        return Err(wasm_bindgen::JsValue::from_str(&e.to_string()));
                    }
                }
                Ok(String::from_utf8_lossy(&cursor.into_inner()).to_string())
            })
            .collect::<Result<_, _>>()?,
    })
}
