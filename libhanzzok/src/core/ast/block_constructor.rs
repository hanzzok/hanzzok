use core::fmt;

use crate::syntax::Token;

use super::InlineObjectNode;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BlockConstructorForm {
    Basic,
    Shortened,
    Bookend,
    Leading,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Clone, Debug)]
pub struct BlockConstructorNode {
    pub(crate) form: BlockConstructorForm,
    pub(crate) name: String,
    pub(crate) main_text: Vec<InlineObjectNode>,
    pub(crate) param: Option<String>,
    pub(crate) multiline_text: Vec<Vec<InlineObjectNode>>,
    pub(crate) tokens: Vec<Token>,
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
