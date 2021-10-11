use core::fmt;

use crate::core::Span;

use super::InlineObjectNode;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BlockConstructorForm {
    Basic,
    Shortened,
    Bookend,
    Leading,
}

#[derive(Clone, Debug)]
pub struct BlockConstructorNode {
    pub form: BlockConstructorForm,
    pub name: String,
    pub main_text: Vec<InlineObjectNode>,
    pub param: Option<String>,
    pub multiline_text: Vec<InlineObjectNode>,
    pub span: Span,
}

impl fmt::Display for BlockConstructorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BlockConstructor(form={:?}, name={}, main_text=[",
            self.form, self.name,
        )?;
        let mut has_written = false;
        for node in &self.main_text {
            if has_written {
                write!(f, ", ")?;
            }
            node.fmt(f)?;
            has_written = true;
        }
        write!(f, "], param={:?}, multiline_text=[", self.param)?;
        let mut has_written = false;
        for node in &self.multiline_text {
            if has_written {
                write!(f, ", ")?;
            }
            node.fmt(f)?;
            has_written = true;
        }
        write!(f, "])")
    }
}
