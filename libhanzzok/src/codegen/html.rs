use std::{
    collections::HashMap,
    io::{self, Write},
    rc::Rc,
};

use super::Context;

#[derive(Clone)]
pub enum HtmlNode {
    Tag(HtmlTagNode),
    Text(String),
    Lazy(Rc<Box<dyn Fn(&Context) -> HtmlNode>>),
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

    pub fn create_lazy(f: impl Fn(&Context) -> HtmlNode + 'static) -> HtmlNode {
        HtmlNode::Lazy(Rc::new(Box::new(f)))
    }

    pub fn into_plain_text(self, context: &Context) -> String {
        match self {
            HtmlNode::Text(text) => text,
            HtmlNode::Tag(HtmlTagNode { children, .. }) => children
                .into_iter()
                .map(|node| node.into_plain_text(context))
                .collect(),
            HtmlNode::Lazy(f) => f(context).into_plain_text(context),
        }
    }

    pub fn evaluate_early(self, context: &Context) -> HtmlNode {
        match self {
            HtmlNode::Lazy(f) => f(context),
            otherwise => otherwise,
        }
    }

    pub fn write(&self, context: &Context, w: &mut impl Write) -> io::Result<()> {
        match self {
            HtmlNode::Tag(node) => node.write(context, w),
            HtmlNode::Text(data) => write!(w, "{}", data),
            HtmlNode::Lazy(f) => f(context).write(context, w),
        }
    }
}

pub struct HtmlTagNodeBuilder {
    pub(crate) tag: String,
    pub(crate) attrs: HashMap<String, String>,
    pub(crate) children: Vec<HtmlNode>,
}

impl HtmlTagNodeBuilder {
    pub fn append(&mut self, child: HtmlNode) -> &mut Self {
        self.children.push(child);
        self
    }
    pub fn append_all(&mut self, children: impl IntoIterator<Item = HtmlNode>) -> &mut Self {
        self.children.extend(children);
        self
    }
    pub fn set_attr(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
        self.attrs
            .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
        self
    }
    pub fn append_attr(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
        let entry = self
            .attrs
            .entry(key.as_ref().to_owned())
            .or_insert("".to_owned());
        *entry = format!("{}{}", entry, value.as_ref());
        self
    }

    pub fn build(&mut self) -> HtmlNode {
        HtmlNode::Tag(HtmlTagNode {
            tag: self.tag.clone(),
            attrs: self.attrs.clone(),
            children: self.children.clone(),
        })
    }
}

#[derive(Clone)]
pub struct HtmlTagNode {
    tag: String,
    attrs: HashMap<String, String>,
    children: Vec<HtmlNode>,
}

impl HtmlTagNode {
    pub fn write(&self, context: &Context, w: &mut impl Write) -> io::Result<()> {
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
            node.write(context, w)?;
        }
        write!(w, "</{}>", &self.tag)
    }
}
