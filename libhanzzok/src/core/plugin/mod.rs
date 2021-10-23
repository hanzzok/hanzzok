use std::rc::Rc;

use crate::api::{BlockConstructorRule, DecoratorRule};

pub use code::code_plugin;
pub use emphasize::emphasize_plugin;
pub use heading::heading_plugin;
pub use list::list_plugin;
pub use math::math_plugin;
pub use quotation::quotation_plugin;
pub use youtube::youtube_plugin;

mod code;
mod emphasize;
mod heading;
mod list;
mod math;
mod quotation;
mod youtube;

pub struct Plugin {
    pub(crate) block_constructors: Vec<Rc<dyn BlockConstructorRule>>,
    pub(crate) decorators: Vec<Rc<dyn DecoratorRule>>,
}

impl Plugin {
    pub fn new() -> Self {
        Plugin {
            block_constructors: Vec::new(),
            decorators: Vec::new(),
        }
    }
    pub fn with_block_constructor(mut self, rule: impl BlockConstructorRule + 'static) -> Self {
        self.block_constructors.push(Rc::new(rule));
        self
    }
    pub fn with_decorator(mut self, rule: impl DecoratorRule + 'static) -> Self {
        self.decorators.push(Rc::new(rule));
        self
    }
}
