use crate::{
    api::BlockConstructorRule,
    codegen::{Context, HtmlNode, Walker},
    core::ast::{BlockConstructorForm, InlineObjectNode},
};

pub struct BlockquoteBlockConstructorRule;

impl BlockConstructorRule for BlockquoteBlockConstructorRule {
    fn name(&self) -> String {
        ">".to_owned()
    }

    fn form(&self) -> crate::core::ast::BlockConstructorForm {
        BlockConstructorForm::Leading
    }

    fn apply(
        &self,
        context: &mut Context,
        _: Vec<InlineObjectNode>,
        _: Option<String>,
        multiline_text: Vec<Vec<InlineObjectNode>>,
    ) -> HtmlNode {
        let mut lines = Vec::new();

        let mut previous = Vec::new();

        for line in multiline_text {
            if line.is_empty() {
                if !previous.is_empty() {
                    lines.push(HtmlNode::create_tag("p", &context.walk(previous)));
                    previous = Vec::new();
                }
            } else {
                previous.extend(line);
            }
        }

        if !previous.is_empty() {
            lines.push(HtmlNode::create_tag("p", &context.walk(previous)));
        }

        HtmlNode::create_tag("blockquote", &lines)
    }
}
