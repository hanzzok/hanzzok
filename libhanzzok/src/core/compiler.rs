use std::collections::HashMap;

use crate::api::BlockConstructorRule;

pub struct Compiler {
    block_constructor_rules: HashMap<String, Box<dyn BlockConstructorRule>>,
}
