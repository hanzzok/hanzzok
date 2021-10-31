use self::{math_block::MathBlockConstructorRule, math_inline::MathDecoratorRule};

use super::Plugin;

mod math_block;
mod math_inline;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn math_plugin() -> Plugin {
    Plugin::new()
        .with_block_constructor(MathBlockConstructorRule)
        .with_decorator(MathDecoratorRule)
}
