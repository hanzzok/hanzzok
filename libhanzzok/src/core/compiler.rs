use std::{collections::HashMap, rc::Rc};

use crate::api::BlockConstructorRule;

use super::Plugin;

pub struct Compiler {
    pub(crate) block_constructor_rules: HashMap<String, Rc<dyn BlockConstructorRule>>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            block_constructor_rules: HashMap::new(),
        }
    }
    pub fn with(mut self, plugin: Plugin) -> Self {
        for block_constructor in plugin.block_constructors {
            self.block_constructor_rules
                .insert(block_constructor.name(), block_constructor);
        }
        self
    }
}
