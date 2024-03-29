use serde_hzdata::HzdataValue;

use crate::{
    api::DecoratorRule,
    codegen::{Context, HtmlNode},
};

pub struct MathDecoratorRule;

impl DecoratorRule for MathDecoratorRule {
    fn name(&self) -> String {
        "math".to_owned()
    }

    fn accept_raw_text(&self) -> bool {
        true
    }

    fn apply(
        &self,
        _context: &mut Context,
        target: Vec<HtmlNode>,
        _param: Option<HzdataValue>,
    ) -> Vec<HtmlNode> {
        vec![HtmlNode::create_tag_builder("span")
            .append_all(target)
            .set_attr("class", "math-inline")
            .build()]
    }
}
