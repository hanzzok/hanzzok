use self::hx::HxBlockConstructorRule;

use super::Plugin;

mod hx;

pub fn heading_plugin() -> Plugin {
    Plugin::new()
        .with_block_constructor(HxBlockConstructorRule { depth: 1 })
        .with_block_constructor(HxBlockConstructorRule { depth: 2 })
        .with_block_constructor(HxBlockConstructorRule { depth: 3 })
        .with_block_constructor(HxBlockConstructorRule { depth: 4 })
        .with_block_constructor(HxBlockConstructorRule { depth: 5 })
        .with_block_constructor(HxBlockConstructorRule { depth: 6 })
}
