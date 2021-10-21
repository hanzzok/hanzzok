use crate::api::BlockConstructorRule;

pub struct Plugin {
    block_constructors: Vec<Box<dyn BlockConstructorRule>>,
}
