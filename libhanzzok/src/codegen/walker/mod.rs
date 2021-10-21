mod block_constructor;
mod decorator_chain;
mod inline_constructor;
mod inline_object;
mod node;
mod text;

pub trait Walker<T> {
    fn walk(&mut self, node: T);
}

impl<W: Walker<T>, T> Walker<Box<T>> for W {
    fn walk(&mut self, node: Box<T>) {
        self.walk(*node)
    }
}

impl<W: Walker<T>, T: Clone> Walker<&'_ T> for W {
    fn walk(&mut self, node: &'_ T) {
        self.walk(node.clone())
    }
}
