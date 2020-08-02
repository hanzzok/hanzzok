use crate::api::Spanned;

/// Abstract Syntax Tree of Sezong document
#[derive(Debug)]
pub enum Ast {
    /// The `BlockConstructor` node
    BlockConstructor(Spanned<BlockConstructor>),
    /// The `InlineObject` node
    InlineObject(Spanned<InlineObject>),
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

impl Into<Ast> for Spanned<InlineObject> {
    fn into(self) -> Ast {
        Ast::InlineObject(self)
    }
}

/// The block constructor node
#[derive(Debug)]
pub struct BlockConstructor {
    pub(crate) name: Spanned<String>,
    pub(crate) input: Option<Spanned<InlineObject>>,
    pub(crate) params: Option<String>,
    pub(crate) body: Option<String>,
}

/// The inline constructor node
#[derive(Debug)]
pub struct InlineConstructor(pub(crate) Box<DecoratorFunction>);

/// The decorator node
#[derive(Debug)]
pub struct Decorator {
    pub(crate) text: Box<Spanned<InlineObject>>,
    pub(crate) functions: Vec<Spanned<DecoratorFunction>>,
}

/// The decorator function node
#[derive(Debug)]
pub struct DecoratorFunction {
    pub(crate) name: String,
    pub(crate) params: Option<String>,
}

/// The plain text node
#[derive(Debug)]
pub struct PlainText {
    pub(crate) text: String,
}
