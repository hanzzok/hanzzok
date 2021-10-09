use core::fmt;

use super::Span;

pub trait Spanned {
    fn span(&self) -> Span;
}

pub trait DisplayWithoutSpan {
    fn fmt_without_span(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
