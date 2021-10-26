use crate::{
    api::DecoratorRule,
    codegen::{Context, HtmlNode},
};

pub struct SystemTextDecoratorRule;

impl DecoratorRule for SystemTextDecoratorRule {
    fn name(&self) -> String {
        "system-text".to_owned()
    }

    fn accept_raw_text(&self) -> bool {
        true
    }

    fn apply(
        &self,
        _context: &mut Context,
        target: Vec<HtmlNode>,
        _param: Option<String>,
    ) -> Vec<HtmlNode> {
        vec![HtmlNode::create_tag_builder("kbd")
            .append(HtmlNode::create_tag("samp", &target))
            .set_attr("class", "system-text")
            .build()]
    }
}
