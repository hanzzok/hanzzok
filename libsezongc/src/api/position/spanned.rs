use crate::api::Span;

/// A trait for struct that has span
pub trait Spanned {
    /// The span of this
    fn span(&self) -> Span;
}
