use self::emphasize::EmphasizeDecoratorRule;

use super::Plugin;

mod emphasize;

pub fn emphasize_plugin() -> Plugin {
    Plugin::new()
        .with_decorator(EmphasizeDecoratorRule {
            name: "bold".to_owned(),
            tag: "b".to_owned(),
        })
        .with_decorator(EmphasizeDecoratorRule {
            name: "italic".to_owned(),
            tag: "i".to_owned(),
        })
        .with_decorator(EmphasizeDecoratorRule {
            name: "underline".to_owned(),
            tag: "u".to_owned(),
        })
        .with_decorator(EmphasizeDecoratorRule {
            name: "strikethrough".to_owned(),
            tag: "s".to_owned(),
        })
}
