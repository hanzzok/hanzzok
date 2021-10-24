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
        context: &mut Context,
        target: Vec<HtmlNode>,
        param: Option<String>,
    ) -> Vec<HtmlNode> {
        vec![HtmlNode::create_tag_builder("span")
            .append_all(target)
            .set_attr("class", "math-inline")
            .build()]
    }
}
