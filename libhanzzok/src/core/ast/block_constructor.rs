use core::fmt;

use crate::syntax::Token;

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
    pub multiline_text: Vec<Vec<InlineObjectNode>>,
    pub tokens: Vec<Token>,
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
        for nodes in &self.multiline_text {
            if has_written {
                write!(f, ", ")?;
            }
            write!(f, "[")?;
            let mut inner_has_written = false;
            for node in nodes {
                if inner_has_written {
                    write!(f, ", ")?;
                }
                node.fmt(f)?;
                inner_has_written = true;
            }
            write!(f, "]")?;
            has_written = true;
        }
        write!(f, "])")
    }
}
