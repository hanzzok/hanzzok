//! Public APIs of [`libsezongc`]
//!
//! [`libsezongc`]: /libsezongc

mod ast;
mod compiler;
mod diagnostic;
mod platform;
mod position;
mod preprocessor;
mod renderable;
mod renderer;
mod rule;
mod source_file;

pub use ast::*;
pub use compiler::*;
pub use diagnostic::*;
pub use platform::*;
pub use position::*;
pub use preprocessor::*;
pub use renderable::*;
pub use renderer::*;
pub use rule::*;
pub use source_file::*;

pub(crate) type ParserSpan<'a> = nom_locate::LocatedSpan<&'a str>;
pub(crate) type ParserResultBase<'a, T> = nom::IResult<ParserSpan<'a>, T, ()>;
pub(crate) type ParserResult<'a, T> = ParserResultBase<'a, Spanned<T>>;

pub(crate) mod parse_prelude {
    pub(crate) use super::ast::*;
    pub(crate) use super::position::*;
    pub(crate) use super::{ParserResult, ParserResultBase, ParserSpan};
}
