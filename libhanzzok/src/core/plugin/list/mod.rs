use self::unordered_list::UnorderedListBlockConstructorRule;

use super::Plugin;

mod unordered_list;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn list_plugin() -> Plugin {
    Plugin::new().with_block_constructor(UnorderedListBlockConstructorRule)
}
