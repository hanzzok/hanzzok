use crate::syntax::Token;

pub trait Raw {
    fn raw(&self) -> Vec<Token>;
}

impl<T: Raw> Raw for Vec<T> {
    fn raw(&self) -> Vec<Token> {
        self.iter().flat_map(|value| value.raw()).collect()
    }
}
