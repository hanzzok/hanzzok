use super::Span;

pub trait Spanned {
    fn span(&self) -> Span;
}
