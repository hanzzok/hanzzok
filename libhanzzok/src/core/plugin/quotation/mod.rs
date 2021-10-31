use self::blockquote::BlockquoteBlockConstructorRule;

use super::Plugin;

mod blockquote;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn quotation_plugin() -> Plugin {
    Plugin::new().with_block_constructor(BlockquoteBlockConstructorRule)
}
