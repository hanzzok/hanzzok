use self::unordered_list::UnorderedListBlockConstructorRule;

use super::Plugin;

mod unordered_list;

pub fn list_plugin() -> Plugin {
    Plugin::new().with_block_constructor(UnorderedListBlockConstructorRule)
}
