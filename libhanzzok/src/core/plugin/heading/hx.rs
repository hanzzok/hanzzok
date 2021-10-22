use crate::{
    api::BlockConstructorRule,
    codegen::{Context, HtmlNode, Walker},
    core::ast::{BlockConstructorForm, InlineObjectNode},
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
        context: &Context,
        main_text: Vec<InlineObjectNode>,
        _param: Option<String>,
        _: Vec<Vec<InlineObjectNode>>,
    ) -> Option<crate::codegen::HtmlNode> {
        Some(HtmlNode::create_tag(
            format!("h{}", self.depth),
            &context.walk(main_text),
        ))
    }
}
