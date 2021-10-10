use core::fmt;

use super::Span;

pub trait Spanned {
    fn span(&self) -> Span;
}

pub trait DisplayWithSpan {
    fn fmt_with_span(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
