use self::{code_block::CodeBlockConstructorRule, code_inline::CodeDecoratorRule};

use super::Plugin;

mod code_block;
mod code_inline;

pub fn code_plugin() -> Plugin {
    Plugin::new()
        .with_block_constructor(CodeBlockConstructorRule)
        .with_decorator(CodeDecoratorRule)
}
