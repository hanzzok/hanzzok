use crate::{
    api::InlineConstructorRule,
    codegen::{Context, HtmlNode},
};

pub struct KeyComboInlineConstructorRule;

impl InlineConstructorRule for KeyComboInlineConstructorRule {
    fn name(&self) -> String {
        "key-combo".to_owned()
    }

    fn apply(
        &self,
        _context: &mut Context,
        param: Option<serde_hzdata::HzdataValue>,
    ) -> Option<HtmlNode> {
        let keys = serde_hzdata::from_value::<Vec<String>>(param?).ok()?;

        Some(
            HtmlNode::create_tag_builder("kbd")
                .append_all(keys.into_iter().enumerate().flat_map(|(i, name)| {
                    let node = HtmlNode::create_tag_builder("kbd")
                        .append(HtmlNode::create_text(name))
                        .set_attr("class", "key")
                        .build();
                    if i > 0 {
                        vec![HtmlNode::create_text(" + "), node]
                    } else {
                        vec![node]
                    }
                }))
                .set_attr("class", "key-combo")
                .build(),
        )
    }
}
