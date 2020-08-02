use crate::api::{LineColumn, ParserResult, ParserResultBase, ParserSpan, Span};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Spanned<T>
where
    T: Debug,
{
    pub value: T,
    pub span: Span,
}

impl<T> Spanned<T>
where
    T: Debug,
{
    pub fn wrap<'a, F>(f: F) -> impl for<'b> Fn(ParserSpan<'a>) -> ParserResult<'a, T>
    where
        F: Fn(ParserSpan<'a>) -> ParserResultBase<'a, T>,
    {
        move |s| {
            let start = LineColumn::create_from(&s);
            let (s, value) = f(s)?;
            let end = LineColumn::create_from(&s);
            Ok((
                s,
                Spanned {
                    value,
                    span: Span { start, end },
                },
            ))
        }
    }

    pub fn wrap_mut<'a, F>(mut f: F) -> impl for<'b> FnMut(ParserSpan<'a>) -> ParserResult<'a, T>
    where
        F: FnMut(ParserSpan<'a>) -> nom::IResult<ParserSpan<'a>, T, ()>,
    {
        move |s| {
            let start = LineColumn::create_from(&s);
            let (s, value) = f(s)?;
            let end = LineColumn::create_from(&s);
            Ok((
                s,
                Spanned {
                    value,
                    span: Span { start, end },
                },
            ))
        }
    }
}

impl<T> Spanned<T>
where
    T: Debug,
{
    /// Maps an Spanned<T> to Spanned<U> by applying a function to a contained value.
    pub fn map<F, U: Debug>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(T) -> U,
    {
        Spanned {
            value: f(self.value),
            span: self.span,
        }
    }
    /// Maps an Spanned<T> to Spanned<U> by applying a function to a self.
    pub fn map_wrapped<F, U: Debug>(self, f: F) -> Spanned<U>
    where
        F: FnOnce(Spanned<T>) -> U,
    {
        Spanned {
            value: f(Spanned {
                value: self.value,
                span: self.span.clone(),
            }),
            span: self.span,
        }
    }
}

impl<T> Spanned<Box<T>>
where
    T: Debug,
{
    /// Tranpose a Spanned of Box into a Box of Spanned
    pub fn transpose(self) -> Box<Spanned<T>> {
        Box::new(Spanned {
            value: *self.value,
            span: self.span,
        })
    }
}
