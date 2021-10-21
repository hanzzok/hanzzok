use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Clone, Debug)]
pub enum HtmlNode {
    Tag(HtmlTagNode),
    Text(String),
}

impl HtmlNode {
    pub fn create_tag(tag: impl AsRef<str>, children: &[HtmlNode]) -> HtmlNode {
        HtmlNode::Tag(HtmlTagNode {
            tag: tag.as_ref().to_owned(),
            attrs: HashMap::new(),
            children: children.to_vec(),
        })
    }
    pub fn create_text(data: impl AsRef<str>) -> HtmlNode {
        HtmlNode::Text(data.as_ref().to_owned())
    }

    pub fn write(&self, w: &mut impl Write) -> io::Result<()> {
        match self {
            HtmlNode::Tag(node) => node.write(w),
            HtmlNode::Text(data) => write!(w, "{}", data),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HtmlTagNode {
    tag: String,
    attrs: HashMap<String, String>,
    children: Vec<HtmlNode>,
}

impl HtmlTagNode {
    pub fn write(&self, w: &mut impl Write) -> io::Result<()> {
        write!(w, "<{}", &self.tag)?;
        for (key, value) in &self.attrs {
            write!(w, " {}=\"{}\"", key, value)?;
        }
        if self.children.len() > 0 {
            write!(w, ">")?;
            for node in &self.children {
                node.write(w)?;
            }
            write!(w, "</{}>", &self.tag)
        } else {
            write!(w, "/>")
        }
    }
}
