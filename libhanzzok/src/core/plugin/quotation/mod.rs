use self::blockquote::BlockquoteBlockConstructorRule;

use super::Plugin;

mod blockquote;

pub fn quotation_plugin() -> Plugin {
    Plugin::new().with_block_constructor(BlockquoteBlockConstructorRule)
}
