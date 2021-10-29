use serde_hzdata::HzdataValue;

use crate::{
    api::BlockConstructorRule,
    codegen::{Context, HtmlNode, Walker},
    core::{
        ast::{BlockConstructorForm, InlineObjectNode},
        plugin::heading::heading_meta::{Heading, HeadingList},
    },
};

pub struct HxBlockConstructorRule {
    pub(super) depth: usize,
}

impl BlockConstructorRule for HxBlockConstructorRule {
    fn name(&self) -> String {
        "#".repeat(self.depth)
    }

    fn form(&self) -> crate::core::ast::BlockConstructorForm {
        BlockConstructorForm::Shortened
    }

    fn apply(
        &self,
        context: &mut Context,
        main_text: Vec<InlineObjectNode>,
        _param: Option<HzdataValue>,
        _: Vec<Vec<InlineObjectNode>>,
    ) -> HtmlNode {
        let main_text = context.walk(main_text);

        let mut meta: HeadingList = context.load_meta_or_default("heading", "list");
        meta.values.push(Heading {
            name: main_text
                .iter()
                .map(|node| node.clone().into_ref(context))
                .collect(),
            depth: self.depth,
        });
        context.save_meta("heading", "list", meta).unwrap();

        HtmlNode::create_tag(format!("h{}", self.depth), &main_text)
    }
}
