use serde_hzdata::HzdataValue;

use crate::{
    api::BlockConstructorRule,
    codegen::{Context, HtmlNode, Walker},
    core::ast::{BlockConstructorForm, InlineObjectNode},
};

pub struct UnorderedListBlockConstructorRule;

impl BlockConstructorRule for UnorderedListBlockConstructorRule {
    fn name(&self) -> String {
        "-".to_owned()
    }

    fn form(&self) -> crate::core::ast::BlockConstructorForm {
        BlockConstructorForm::Leading
    }

    fn apply(
        &self,
        context: &mut Context,
        _: Vec<InlineObjectNode>,
        _: Option<HzdataValue>,
        multiline_text: Vec<Vec<InlineObjectNode>>,
    ) -> HtmlNode {
        HtmlNode::create_tag(
            "ul",
            &multiline_text
                .iter()
                .map(|line| HtmlNode::create_tag("li", &context.walk(line)))
                .collect::<Vec<_>>(),
        )
    }
}
