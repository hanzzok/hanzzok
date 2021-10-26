use self::{
    key::KeyInlineConstructorRule, key_combo::KeyComboInlineConstructorRule,
    system_text::SystemTextDecoratorRule,
};

use super::Plugin;

mod key;
mod key_combo;
mod system_text;

pub fn input_guide_plugin() -> Plugin {
    Plugin::new()
        .with_decorator(SystemTextDecoratorRule)
        .with_inline_constructor(KeyInlineConstructorRule)
        .with_inline_constructor(KeyComboInlineConstructorRule)
}
