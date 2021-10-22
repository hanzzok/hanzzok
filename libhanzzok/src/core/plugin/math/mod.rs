use self::math_block::MathBlockConstructorRule;

use super::Plugin;

mod math_block;

pub fn math_plugin() -> Plugin {
    Plugin::new().with_block_constructor(MathBlockConstructorRule)
}
