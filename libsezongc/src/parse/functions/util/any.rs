use super::*;
use crate::parse::{ParseError, ParseResult, Parser};

pub(crate) trait List<T> {
    fn choice(&mut self, parser: &mut Parser<'_>, require: bool) -> ParseResult<T>;
}

macro_rules! next {
    ($tag:tt 0 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 1 $($rest)*) };
    ($tag:tt 1 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 2 $($rest)*) };
    ($tag:tt 2 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 3 $($rest)*) };
    ($tag:tt 3 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 4 $($rest)*) };
    ($tag:tt 4 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 5 $($rest)*) };
    ($tag:tt 5 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 6 $($rest)*) };
    ($tag:tt 6 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 7 $($rest)*) };
    ($tag:tt 7 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 8 $($rest)*) };
    ($tag:tt 8 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 9 $($rest)*) };
    ($tag:tt 9 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 10 $($rest)*) };
    ($tag:tt 10 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 11 $($rest)*) };
    ($tag:tt 11 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 12 $($rest)*) };
    ($tag:tt 12 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 13 $($rest)*) };
    ($tag:tt 13 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 14 $($rest)*) };
    ($tag:tt 14 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 15 $($rest)*) };
    ($tag:tt 15 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 16 $($rest)*) };
    ($tag:tt 16 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 17 $($rest)*) };
    ($tag:tt 17 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 18 $($rest)*) };
    ($tag:tt 18 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 19 $($rest)*) };
    ($tag:tt 19 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 20 $($rest)*) };
    ($tag:tt 20 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 21 $($rest)*) };
    ($tag:tt 21 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 22 $($rest)*) };
    ($tag:tt 22 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 23 $($rest)*) };
    ($tag:tt 23 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 24 $($rest)*) };
    ($tag:tt 24 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 25 $($rest)*) };
    ($tag:tt 25 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 26 $($rest)*) };
    ($tag:tt 26 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 27 $($rest)*) };
    ($tag:tt 27 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 28 $($rest)*) };
    ($tag:tt 28 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 29 $($rest)*) };
    ($tag:tt 29 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 30 $($rest)*) };
    ($tag:tt 30 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 31 $($rest)*) };
    ($tag:tt 31 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 32 $($rest)*) };
    ($tag:tt 32 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 33 $($rest)*) };
    ($tag:tt 33 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 34 $($rest)*) };
    ($tag:tt 34 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 35 $($rest)*) };
    ($tag:tt 35 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 36 $($rest)*) };
    ($tag:tt 36 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 37 $($rest)*) };
    ($tag:tt 37 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 38 $($rest)*) };
    ($tag:tt 38 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 39 $($rest)*) };
    ($tag:tt 39 $submacro:ident @params $($rest:tt)*) => { $submacro!(@$tag 40 $($rest)*) };
    ($any:tt) => {};
}

macro_rules! impl_list {
    ($first:ident, $($generic:ident),*) => {
        impl_list!(@iterate $first; $($generic)*);
    };

    (@iterate $($current:ident)*; $head:ident $($tail:ident)*) => {
        impl_list!(@impl $($current)*);
        impl_list!(@iterate $($current)* $head; $($tail)*);
    };
    (@iterate $($current:ident)*;) => {
        impl_list!(@impl $($current)*);
    };

    (@impl $($generics:tt)*) => {
        impl<
            T,
            $($generics: FnMut(&mut Parser<'_>) -> ParseResult<T>,)*
        > List<T> for ($($generics,)*)
        {
            fn choice(&mut self, parser: &mut Parser<'_>, require: bool) -> ParseResult<T> {
                let start_cursor = parser.cursor;

                impl_list!(@body 0 self parser start_cursor require $($generics)*)
            }
        }
    };
    (@body $index:tt $self:ident $parser:ident $start_cursor:ident $require:ident $generic:tt $($generics:tt)*) => {
        match $self.$index($parser) {
            ok @ Ok(_) => ok,
            Err(ParseError::Unmatched(_)) => {
                $parser.cursor = $start_cursor;
                next!(body $index impl_list @params $self $parser $start_cursor $require $($generics)*)
            }
            Err(error) => error.into(),
        }
    };
    (@body $index:tt $self:ident $parser:ident $start_cursor:ident $require:ident) => {
        if $require {
            unexpected($parser)
        } else {
            unmatched($parser)
        }
    };
}

impl_list!(
    F01, F02, F03, F04, F05, F06, F07, F08, F09, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19,
    F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31, F32, F33, F34, F35, F36, F37, F38,
    F39, F40
);

fn any_raw<T, F>(mut functions: F, require: bool) -> impl FnMut(&mut Parser<'_>) -> ParseResult<T>
where
    F: List<T>,
{
    move |parser| functions.choice(parser, require)
}

pub(crate) fn any<T, F>(mut functions: F) -> impl FnMut(&mut Parser<'_>) -> ParseResult<T>
where
    F: List<T>,
{
    move |parser| functions.choice(parser, false)
}

pub(crate) fn any_require<T, F>(mut functions: F) -> impl FnMut(&mut Parser<'_>) -> ParseResult<T>
where
    F: List<T>,
{
    move |parser| functions.choice(parser, true)
}
