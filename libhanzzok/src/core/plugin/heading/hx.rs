use crate::{
    api::BlockConstructorRule,
    codegen::{Context, HtmlNode, Walker},
    core::ast::BlockConstructorForm,
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
        main_text: Vec<crate::core::ast::InlineObjectNode>,
        param: Option<String>,
        multiline_text: Vec<crate::core::ast::InlineObjectNode>,
    ) -> Option<crate::codegen::HtmlNode> {
        Some(HtmlNode::create_tag(
            format!("h{}", self.depth),
            &main_text
                .iter()
                .flat_map(|node| context.walk(node))
                .collect::<Vec<_>>(),
        ))
    }
}
