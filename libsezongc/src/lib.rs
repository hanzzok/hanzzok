//! A library to create sezong compiler.
//!
//! # Architecture
//!
//! Doing the following tasks, by the order, to compile your documents.
//!
//! - [Preprocess]
//! - [Parse]
//! - [Postprocess]
//!
//! ## Preprocess
//!
//! Before [Parse], modify the original text of the document to make
//! the users comfortable.
//!
//! See [`Preprocessor`] trait for more informations.
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
// TODO: temporarily warn, should be deny
#![warn(missing_docs, unsafe_code)]
#![warn(
    unreachable_pub,
    anonymous_parameters,
    elided_lifetimes_in_paths,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
#![warn(clippy::all)]

pub mod api;
mod parse;

pub use api::Compiler;
