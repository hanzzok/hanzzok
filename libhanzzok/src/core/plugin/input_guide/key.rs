use crate::{
    api::InlineConstructorRule,
    codegen::{Context, HtmlNode},
};

pub struct KeyInlineConstructorRule;

impl InlineConstructorRule for KeyInlineConstructorRule {
    fn name(&self) -> String {
        "key".to_owned()
    }

    fn apply(
        &self,
        _context: &mut Context,
        param: Option<serde_hzdata::HzdataValue>,
    ) -> Option<HtmlNode> {
        let name = serde_hzdata::from_value::<String>(param?).ok()?;

        Some(
            HtmlNode::create_tag_builder("kbd")
                .append(HtmlNode::create_text(name))
                .set_attr("class", "key")
                .build(),
        )
    }
}
