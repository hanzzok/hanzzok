//! A library to create sezong compiler.
//!
//! # Architecture
//!
//! Doing the following tasks, by the order, to compile your documents.
//!
//! - [Preprocess]
//! - [Tokenize]
//! - [Parse]
//! - [Postprocess]
//!
//! ## Preprocess
//!
//! Before [Tokenize], modify the original text of the document to make
//! the users comfortable.
//!
//! See [`Preprocessor`] trait for more informations.
//!
//! ## Tokenize
//!
//! ## Parse
//!
//! ## Postprocess
//!
//! After [Parse], Change a little bit of text contents.
//!
//! [Preprocess]: #preprocess
//! [Tokenize]: #tokenize
//! [Parse]: #parse
//! [Postprocess]: #postprocess
//! [`Preprocessor`]: /libsezongc/api/trait.Preprocessor.html
#![deny(missing_docs, unsafe_code)]
#![warn(
    unreachable_pub,
    anonymous_parameters,
    elided_lifetimes_in_paths,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

pub mod api;
mod core;
mod parse;
mod tokenize;

pub use api::Compiler;
pub use parse::{ParseError, ParseResult, Parser};
pub use tokenize::Tokenizer;
