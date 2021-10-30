use serde_hzdata::HzdataValue;

use crate::{
    api::DecoratorRule,
    codegen::{Context, HtmlNode},
    core::plugin::link::link_ref_meta::LinkRefMeta,
};

pub struct LinkRefDecoratorRule;

impl DecoratorRule for LinkRefDecoratorRule {
    fn name(&self) -> String {
        "link-ref".to_owned()
    }

    fn apply(
        &self,
        _context: &mut Context,
        target: Vec<HtmlNode>,
        _param: Option<HzdataValue>,
    ) -> Vec<HtmlNode> {
        vec![HtmlNode::create_lazy(move |context| {
            let name: String = target
                .iter()
                .map(|node| node.clone().into_plain_text(context))
                .collect();
            let meta: LinkRefMeta = context.load_meta_or_default("link", "ref");
            let link = match meta.get(&name) {
                Some(link) => link,
                None => return HtmlNode::Collection(target.clone()),
            };
            HtmlNode::create_tag_builder("a")
                .append_all(target.clone())
                .set_attr("href", link)
                .build()
        })]
    }
}
