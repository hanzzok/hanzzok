use crate::{
    api::DecoratorRule,
    codegen::{Context, HtmlNode},
};

pub struct CodeDecoratorRule;

impl DecoratorRule for CodeDecoratorRule {
    fn name(&self) -> String {
        "code".to_owned()
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
        vec![HtmlNode::create_tag("code", &target)]
    }
}
