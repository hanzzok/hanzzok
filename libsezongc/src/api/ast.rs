use crate::api::{Span, Spanned, Token};

/// Abstract Syntax Tree of Sezong document
#[derive(Debug)]
pub enum Ast {
    /// The `BlockConstructor` node
    BlockConstructor(BlockConstructor),
    /// The `InlineObject` node
    InlineObject(InlineObject),
}

impl Spanned for Ast {
    fn span(&self) -> Span {
        match self {
            Ast::BlockConstructor(BlockConstructor { span, .. }) => span.clone(),
            Ast::InlineObject(object) => object.span(),
        }
    }
}

/// The `InlineObject` node
#[derive(Debug)]
pub enum InlineObject {
    /// The `InlineConstructor` node
    InlineConstructor(InlineConstructor),
    /// The `Decorator` node
    Decorator(Decorator),
    /// The `PlainText` node
    PlainText(PlainText),
}

impl Spanned for InlineObject {
    fn span(&self) -> Span {
        match self {
            InlineObject::InlineConstructor(InlineConstructor { span, .. }) => span,
            InlineObject::Decorator(Decorator { span, .. }) => span,
            InlineObject::PlainText(PlainText { span, .. }) => span,
        }
        .clone()
    }
}

impl Into<Ast> for InlineObject {
    fn into(self) -> Ast {
        Ast::InlineObject(self)
    }
}

/// The block constructor node
#[derive(Debug)]
pub struct BlockConstructor {
    pub(crate) name: Token,
    pub(crate) span: Span,
}

/// The inline constructor node
#[derive(Debug)]
pub struct InlineConstructor {
    pub(crate) constructor_function: Box<DecoratorFunction>,
    pub(crate) span: Span,
}

/// The decorator node
#[derive(Debug)]
pub struct Decorator {
    pub(crate) text: Box<InlineObject>,
    pub(crate) functions: Vec<DecoratorFunction>,
    pub(crate) span: Span,
}

/// The decorator function node
#[derive(Debug)]
pub struct DecoratorFunction {
    pub(crate) name: Token,
    pub(crate) params: Option<Span>,
    pub(crate) span: Span,
}

/// The plain text node
#[derive(Debug)]
pub struct PlainText {
    pub(crate) span: Span,
}
