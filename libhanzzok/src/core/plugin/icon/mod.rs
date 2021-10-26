use self::icon::IconInlineConstructorRule;

use super::Plugin;

mod icon;

pub fn icon_plugin() -> Plugin {
    Plugin::new().with_inline_constructor(IconInlineConstructorRule)
}
