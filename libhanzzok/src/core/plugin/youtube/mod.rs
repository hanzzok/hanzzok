use self::youtube_embed::YouTubeEmbedBlockConstructorRule;

use super::Plugin;

mod youtube_embed;

pub fn youtube_plugin() -> Plugin {
    Plugin::new().with_block_constructor(YouTubeEmbedBlockConstructorRule)
}
