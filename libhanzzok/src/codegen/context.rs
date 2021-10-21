use std::{
    collections::VecDeque,
    io::{self, Write},
};

use super::html::HtmlNode;

pub struct Context {
    children: VecDeque<HtmlNode>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            children: VecDeque::new(),
        }
    }

    pub fn push(&mut self, node: HtmlNode) {
        self.children.push_back(node)
    }

    pub fn pop(&mut self) -> Option<HtmlNode> {
        self.children.pop_back()
    }

    pub fn write(&self, w: &mut impl Write) -> io::Result<()> {
        for node in &self.children {
            node.write(w)?;
        }
        Ok(())
    }
}
