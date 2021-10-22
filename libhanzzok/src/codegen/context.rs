use crate::core::Compiler;

pub struct Context<'a> {
    pub(crate) compiler: &'a Compiler,
    pub(crate) is_in_container: bool,
}

impl<'a> Context<'a> {
    pub fn new(compiler: &'a Compiler) -> Self {
        Context {
            compiler,
            is_in_container: false,
        }
    }

    pub fn in_container(&self) -> Self {
        Context {
            compiler: self.compiler,
            is_in_container: true,
        }
    }
}
