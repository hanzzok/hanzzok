use crate::{
    codegen::{Context, HtmlNode},
    core::ast::{BlockConstructorForm, InlineObjectNode},
};

pub trait BlockConstructorRule {
    fn name(&self) -> String;

    fn form(&self) -> BlockConstructorForm;

    fn accept_raw_multiline(&self) -> bool {
        false
    }

    fn apply(
        &self,
        context: &mut Context,
        main_text: Vec<InlineObjectNode>,
        param: Option<String>,
        multiline_text: Vec<Vec<InlineObjectNode>>,
    ) -> HtmlNode;
}
