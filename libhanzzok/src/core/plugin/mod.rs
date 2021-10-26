use std::rc::Rc;

use crate::api::{BlockConstructorRule, DecoratorRule, InlineConstructorRule};

pub use code::code_plugin;
pub use emphasize::emphasize_plugin;
pub use heading::heading_plugin;
pub use icon::icon_plugin;
pub use input_guide::input_guide_plugin;
pub use list::list_plugin;
pub use math::math_plugin;
pub use quotation::quotation_plugin;
pub use youtube::youtube_plugin;

mod code;
mod emphasize;
mod heading;
mod icon;
mod input_guide;
mod list;
mod math;
mod quotation;
mod youtube;

pub struct Plugin {
    pub(crate) block_constructors: Vec<Rc<dyn BlockConstructorRule>>,
    pub(crate) decorators: Vec<Rc<dyn DecoratorRule>>,
    pub(crate) inline_constructors: Vec<Rc<dyn InlineConstructorRule>>,
}

impl Plugin {
    pub fn new() -> Self {
        Plugin {
            block_constructors: Vec::new(),
            decorators: Vec::new(),
            inline_constructors: Vec::new(),
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
    pub fn with_inline_constructor(mut self, rule: impl InlineConstructorRule + 'static) -> Self {
        self.inline_constructors.push(Rc::new(rule));
        self
    }
}
