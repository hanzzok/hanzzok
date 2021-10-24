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
    pub fn create_tag_builder(tag: impl AsRef<str>) -> HtmlTagNodeBuilder {
        HtmlTagNodeBuilder {
            tag: tag.as_ref().to_owned(),
            attrs: HashMap::new(),
            children: Vec::new(),
        }
    }
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

pub struct HtmlTagNodeBuilder {
    tag: String,
    attrs: HashMap<String, String>,
    children: Vec<HtmlNode>,
}

impl HtmlTagNodeBuilder {
    pub fn append(mut self, child: HtmlNode) -> Self {
        self.children.push(child);
        self
    }
    pub fn append_all(mut self, children: impl IntoIterator<Item = HtmlNode>) -> Self {
        self.children.extend(children);
        self
    }
    pub fn set_attr(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.attrs
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
        self
    }
    pub fn append_attr(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        let entry = self
            .attrs
            .entry(key.as_ref().to_owned())
            .or_insert("".to_owned());
        *entry = format!("{}{}", entry, value.as_ref());
        self
    }

    pub fn build(self) -> HtmlNode {
        HtmlNode::Tag(HtmlTagNode {
            tag: self.tag,
            attrs: self.attrs,
            children: self.children,
        })
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

        let mut attrs: Vec<_> = self.attrs.iter().collect();
        attrs.sort();

        for (key, value) in attrs {
            if value.is_empty() {
                write!(w, " {}", key)?;
            } else {
                write!(w, " {}=\"{}\"", key, value)?;
            }
        }
        write!(w, ">")?;
        for node in &self.children {
            node.write(w)?;
        }
        write!(w, "</{}>", &self.tag)
    }
}
