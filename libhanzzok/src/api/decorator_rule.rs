use crate::codegen::{Context, HtmlNode};

pub trait DecoratorRule {
    fn name(&self) -> String;

    fn accept_raw_text(&self) -> bool {
        false
    }

    fn apply(
        &self,
        context: &mut Context,
        target: Vec<HtmlNode>,
        param: Option<String>,
    ) -> Vec<HtmlNode>;
}
