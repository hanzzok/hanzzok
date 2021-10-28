use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_hzdata::HzdataValue;

use crate::core::Compiler;

use super::HtmlNode;

pub struct Context<'a> {
    pub(crate) compiler: &'a Compiler,
    meta: HashMap<String, HzdataValue>,
    html_nodes_id_counter: usize,
    html_nodes: HashMap<usize, HtmlNode>,
}

impl<'a> Context<'a> {
    pub fn new(compiler: &'a Compiler) -> Self {
        Context {
            compiler,
            meta: HashMap::new(),
            html_nodes_id_counter: 0,
            html_nodes: HashMap::new(),
        }
    }

    pub fn load_meta<'de, T>(&self, plugin: impl AsRef<str>, name: impl AsRef<str>) -> Option<T>
    where
        T: Deserialize<'de>,
    {
        self.meta
            .get(&format!("{}::{}", plugin.as_ref(), name.as_ref()))
            .and_then(|v| serde_hzdata::from_value(v.clone()).ok())
    }

    pub fn load_meta_or_default<'de, T>(&self, plugin: impl AsRef<str>, name: impl AsRef<str>) -> T
    where
        T: Deserialize<'de>,
        T: Default,
    {
        self.meta
            .get(&format!("{}::{}", plugin.as_ref(), name.as_ref()))
            .and_then(|v| serde_hzdata::from_value(v.clone()).ok())
            .unwrap_or_default()
    }

    pub fn save_meta<T>(
        &mut self,
        plugin: impl AsRef<str>,
        name: impl AsRef<str>,
        meta: T,
    ) -> Result<(), serde_hzdata::Error<'static>>
    where
        T: Serialize,
    {
        self.meta.insert(
            format!("{}::{}", plugin.as_ref(), name.as_ref()),
            serde_hzdata::to_value(meta)?,
        );

        Ok(())
    }

    pub fn save_html_node(&mut self, node: HtmlNode) -> usize {
        let id = self.html_nodes_id_counter;
        self.html_nodes.insert(id, node);
        self.html_nodes_id_counter += 1;
        id
    }

    pub fn load_html_node(&self, id: usize) -> Option<&HtmlNode> {
        self.html_nodes.get(&id)
    }
}
