use self::{
    define_link_ref::DefineLinkRefBlockConstructorRule, link::LinkDecoratorRule,
    link_ref::LinkRefDecoratorRule,
};

use super::Plugin;

mod define_link_ref;
mod link;
mod link_ref;
mod link_ref_meta;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn link_plugin() -> Plugin {
    Plugin::new()
        .with_decorator(LinkDecoratorRule)
        .with_decorator(LinkRefDecoratorRule)
        .with_block_constructor(DefineLinkRefBlockConstructorRule)
}
