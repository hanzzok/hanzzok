use crate::{
    api::BlockConstructorRule,
    codegen::{Context, HtmlNode, Walker},
    core::ast::{BlockConstructorForm, InlineObjectNode},
};

pub struct MathBlockConstructorRule;

impl BlockConstructorRule for MathBlockConstructorRule {
    fn name(&self) -> String {
        "$$".to_owned()
    }

    fn form(&self) -> crate::core::ast::BlockConstructorForm {
        BlockConstructorForm::Bookend
    }

    fn apply(
        &self,
        context: &Context,
        _: Vec<InlineObjectNode>,
        _: Option<String>,
        multiline_text: Vec<Vec<InlineObjectNode>>,
    ) -> Option<crate::codegen::HtmlNode> {
        Some(
            HtmlNode::create_tag_builder("div")
                .append_all(context.walk(multiline_text))
                .set_attr("class", "math-block")
                .build(),
        )
    }
}
