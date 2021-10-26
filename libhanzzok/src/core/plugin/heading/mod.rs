use self::{hx::HxBlockConstructorRule, toc::TocBlockConstructorRule};

use super::Plugin;

mod heading_meta;
mod hx;
mod toc;

pub fn heading_plugin() -> Plugin {
    Plugin::new()
        .with_block_constructor(HxBlockConstructorRule { depth: 1 })
        .with_block_constructor(HxBlockConstructorRule { depth: 2 })
        .with_block_constructor(HxBlockConstructorRule { depth: 3 })
        .with_block_constructor(HxBlockConstructorRule { depth: 4 })
        .with_block_constructor(HxBlockConstructorRule { depth: 5 })
        .with_block_constructor(HxBlockConstructorRule { depth: 6 })
        .with_block_constructor(TocBlockConstructorRule)
}
