use crate::{
    api::InlineConstructorRule,
    codegen::{Context, HtmlNode},
};

pub struct IconInlineConstructorRule;

impl InlineConstructorRule for IconInlineConstructorRule {
    fn name(&self) -> String {
        "icon".to_owned()
    }

    fn apply(
        &self,
        _context: &mut Context,
        param: Option<serde_hzdata::HzdataValue>,
    ) -> Option<HtmlNode> {
        let src = serde_hzdata::from_value::<String>(param?).ok()?;

        Some(
            HtmlNode::create_tag_builder("img")
                .set_attr("src", src)
                .set_attr("class", "icon")
                .build(),
        )
    }
}
