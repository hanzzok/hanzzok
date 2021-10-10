use core::fmt;

use crate::core::Span;

use super::InlineObjectNode;

#[derive(Clone, Debug)]
pub struct DecoratorChainNode {
    pub span: Span,
    pub main_text: Box<InlineObjectNode>,
    pub decorators: Vec<DecoratorNode>,
}

impl fmt::Display for DecoratorChainNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DecoratorChain(main_text={}, decorators=[",
            self.main_text
        )?;
        let mut has_written = false;
        for decorator in &self.decorators {
            if has_written {
                write!(f, ", ")?;
            }
            decorator.fmt(f)?;
            has_written = true;
        }

        write!(f, "])")
    }
}

#[derive(Clone, Debug)]
pub struct DecoratorNode {
    pub span: Span,
    pub name: String,
    pub params: Option<String>,
}

impl fmt::Display for DecoratorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Decorator(name={}{})",
            self.name,
            match &self.params {
                Some(v) => format!(", params={:?}", v),
                None => "".to_owned(),
            }
        )
    }
}
