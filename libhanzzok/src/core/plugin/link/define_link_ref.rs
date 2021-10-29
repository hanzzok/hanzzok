use serde_hzdata::HzdataValue;

use crate::{
    api::BlockConstructorRule,
    codegen::{Context, HtmlNode, Walker},
    core::{
        ast::{BlockConstructorForm, InlineObjectNode},
        plugin::link::link_ref_meta::LinkRefMeta,
    },
};

pub struct DefineLinkRefBlockConstructorRule;

impl BlockConstructorRule for DefineLinkRefBlockConstructorRule {
    fn name(&self) -> String {
        "define-link-ref".to_owned()
    }

    fn form(&self) -> crate::core::ast::BlockConstructorForm {
        BlockConstructorForm::Basic
    }

    fn apply(
        &self,
        context: &mut Context,
        main_text: Vec<InlineObjectNode>,
        param: Option<HzdataValue>,
        _: Vec<Vec<InlineObjectNode>>,
    ) -> HtmlNode {
        let mut meta: LinkRefMeta = context.load_meta_or_default("link", "ref");
        if let Some(link) = param.and_then(|value| serde_hzdata::from_value::<String>(value).ok()) {
            let name: String = context
                .walk(main_text)
                .iter()
                .map(|node| node.clone().into_plain_text(context))
                .collect();
            meta.insert(name.trim().to_owned(), link);
            context.save_meta("link", "ref", meta).unwrap();
        }

        HtmlNode::Empty
    }
}
