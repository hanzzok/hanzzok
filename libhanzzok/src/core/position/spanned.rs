use core::fmt;

use super::Span;

pub trait Spanned {
    fn span(&self) -> Span;
}

impl<T: Spanned> Spanned for &T {
    fn span(&self) -> Span {
        <T as Spanned>::span(self).clone()
    }
}

pub trait DisplayWithSpan {
    fn fmt_with_span(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
