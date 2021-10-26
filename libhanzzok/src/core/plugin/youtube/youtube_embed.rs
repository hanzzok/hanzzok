use crate::{
    api::BlockConstructorRule,
    codegen::{Context, HtmlNode},
    core::ast::{BlockConstructorForm, InlineObjectNode, Raw},
};

pub struct YouTubeEmbedBlockConstructorRule;

impl BlockConstructorRule for YouTubeEmbedBlockConstructorRule {
    fn name(&self) -> String {
        "youtube".to_owned()
    }

    fn form(&self) -> crate::core::ast::BlockConstructorForm {
        BlockConstructorForm::Basic
    }

    fn apply(
        &self,
        _context: &mut Context,
        main_text: Vec<InlineObjectNode>,
        _: Option<String>,
        _: Vec<Vec<InlineObjectNode>>,
    ) -> HtmlNode {
        HtmlNode::create_tag_builder("iframe")
            .set_attr(
                "src",
                format!(
                    "https://www.youtube.com/embed/{}",
                    main_text
                        .raw()
                        .iter()
                        .map(|t| t.text.clone())
                        .collect::<String>()
                ),
            )
            .set_attr("width", "560")
            .set_attr("height", "315")
            .set_attr("title", "YouTube video player")
            .set_attr("frameborder", "0")
            .set_attr("allow", "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture")
            .set_attr("allowfullscreen","")
            .build()
    }
}
