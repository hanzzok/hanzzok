use self::code_block::CodeBlockConstructorRule;

use super::Plugin;

mod code_block;

pub fn code_plugin() -> Plugin {
    Plugin::new().with_block_constructor(CodeBlockConstructorRule)
}
