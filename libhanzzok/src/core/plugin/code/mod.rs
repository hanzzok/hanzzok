use self::{code_block::CodeBlockConstructorRule, code_inline::CodeDecoratorRule};

use super::Plugin;

mod code_block;
mod code_inline;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn code_plugin() -> Plugin {
    Plugin::new()
        .with_block_constructor(CodeBlockConstructorRule)
        .with_decorator(CodeDecoratorRule)
}
