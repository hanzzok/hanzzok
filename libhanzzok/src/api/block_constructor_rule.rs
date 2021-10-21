use crate::{
    codegen::HtmlNode,
    core::ast::{BlockConstructorForm, InlineObjectNode},
};

pub trait BlockConstructorRule {
    fn name(&self) -> String;

    fn form(&self) -> BlockConstructorForm;

    fn apply(
        &self,
        main_text: Vec<InlineObjectNode>,
        param: Option<String>,
        multiline_text: Vec<InlineObjectNode>,
    ) -> Option<HtmlNode>;
}
