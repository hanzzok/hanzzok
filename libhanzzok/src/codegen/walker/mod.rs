use super::HtmlNode;

mod block_constructor;
mod decorator_chain;
mod inline_constructor;
mod inline_object;
mod node;
mod text;

pub trait Walker<T> {
    fn walk(&self, node: T) -> Vec<HtmlNode>;
}

impl<W: Walker<T>, T> Walker<Box<T>> for W {
    fn walk(&self, node: Box<T>) -> Vec<HtmlNode> {
        self.walk(*node)
    }
}

impl<W: Walker<T>, T: Clone> Walker<&'_ T> for W {
    fn walk(&self, node: &'_ T) -> Vec<HtmlNode> {
        self.walk(node.clone())
    }
}
