use std::rc::Rc;

use crate::api::BlockConstructorRule;

pub use code::code_plugin;
pub use heading::heading_plugin;
pub use list::list_plugin;
pub use math::math_plugin;
pub use quotation::quotation_plugin;
pub use youtube::youtube_plugin;

mod code;
mod heading;
mod list;
mod math;
mod quotation;
mod youtube;

pub struct Plugin {
    pub(crate) block_constructors: Vec<Rc<dyn BlockConstructorRule>>,
}

impl Plugin {
    pub fn new() -> Self {
        Plugin {
            block_constructors: Vec::new(),
        }
    }
    pub fn with_block_constructor(mut self, rule: impl BlockConstructorRule + 'static) -> Self {
        self.block_constructors.push(Rc::new(rule));
        self
    }
}
