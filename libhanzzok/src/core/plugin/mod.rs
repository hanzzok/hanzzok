use crate::api::BlockConstructorRule;

pub use heading::heading_plugin;
pub use list::list_plugin;
pub use quotation::quotation_plugin;

mod heading;
mod list;
mod quotation;

pub struct Plugin {
    pub(crate) block_constructors: Vec<Box<dyn BlockConstructorRule>>,
}

impl Plugin {
    pub fn new() -> Self {
        Plugin {
            block_constructors: Vec::new(),
        }
    }
    pub fn with_block_constructor(mut self, rule: impl BlockConstructorRule + 'static) -> Self {
        self.block_constructors.push(Box::new(rule));
        self
    }
}
