use serde_hzdata::HzdataValue;

use crate::{
    api::DecoratorRule,
    codegen::{Context, HtmlNode},
};

pub struct LinkDecoratorRule;

impl DecoratorRule for LinkDecoratorRule {
    fn name(&self) -> String {
        "link".to_owned()
    }

    fn apply(
        &self,
        _context: &mut Context,
        target: Vec<HtmlNode>,
        param: Option<HzdataValue>,
    ) -> Vec<HtmlNode> {
        let link: String = match param.and_then(|param| serde_hzdata::from_value(param).ok()) {
            Some(link) => link,
            None => return target,
        };
        vec![HtmlNode::create_tag_builder("a")
            .append_all(target)
            .set_attr("href", link)
            .build()]
    }
}
