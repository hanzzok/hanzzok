use self::youtube_embed::YouTubeEmbedBlockConstructorRule;

use super::Plugin;

mod youtube_embed;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn youtube_plugin() -> Plugin {
    Plugin::new().with_block_constructor(YouTubeEmbedBlockConstructorRule)
}
