use crate::core::Compiler;

pub struct Context<'a> {
    pub(crate) compiler: &'a Compiler,
}

impl<'a> Context<'a> {
    pub fn new(compiler: &'a Compiler) -> Self {
        Context { compiler }
    }
}
