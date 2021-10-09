use core::fmt;

pub use text::TextNode;

mod text;

#[derive(Clone, Debug)]
pub enum HanzzokAstNode {
    Text(TextNode),
}

impl fmt::Display for HanzzokAstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HanzzokAstNode::Text(node) => node.fmt(f),
        }
    }
}
