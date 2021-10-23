use crate::codegen::{Context, HtmlNode};

pub trait DecoratorRule {
    fn name(&self) -> String;

    fn apply(
        &self,
        context: &Context,
        target: Vec<HtmlNode>,
        param: Option<String>,
    ) -> Vec<HtmlNode>;
}
