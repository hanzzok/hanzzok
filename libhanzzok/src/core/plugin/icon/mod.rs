use self::icon::IconInlineConstructorRule;

use super::Plugin;

mod icon;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn icon_plugin() -> Plugin {
    Plugin::new().with_inline_constructor(IconInlineConstructorRule)
}
