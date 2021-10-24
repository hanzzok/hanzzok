use std::collections::HashMap;

use serde_hzdata::HzdataValue;

use crate::core::{Compiler, Plugin};

pub struct Context<'a> {
    pub(crate) compiler: &'a Compiler,
    pub meta: HashMap<(Plugin, String), HzdataValue>,
}

impl<'a> Context<'a> {
    pub fn new(compiler: &'a Compiler) -> Self {
        Context {
            compiler,
            meta: HashMap::new(),
        }
    }
}
