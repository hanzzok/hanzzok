use crate::{
    api::DecoratorRule,
    codegen::{Context, HtmlNode},
};

pub struct EmphasizeDecoratorRule {
    pub(super) name: String,
    pub(super) tag: String,
}

impl DecoratorRule for EmphasizeDecoratorRule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn apply(
        &self,
        context: &mut Context,
        target: Vec<HtmlNode>,
        param: Option<String>,
    ) -> Vec<HtmlNode> {
        vec![HtmlNode::create_tag(&self.tag, &target)]
    }
}
