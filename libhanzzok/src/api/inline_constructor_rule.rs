use serde_hzdata::HzdataValue;

use crate::codegen::{Context, HtmlNode};

pub trait InlineConstructorRule {
    fn name(&self) -> String;

    fn apply(&self, context: &mut Context, param: Option<HzdataValue>) -> Option<HtmlNode>;
}
