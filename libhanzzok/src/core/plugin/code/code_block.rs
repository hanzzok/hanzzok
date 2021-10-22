use v_htmlescape::escape;

use crate::{
    api::BlockConstructorRule,
    codegen::{Context, HtmlNode},
    core::ast::{BlockConstructorForm, InlineObjectNode, Raw},
    syntax::{Token, TokenKind},
};

pub struct CodeBlockConstructorRule;

impl BlockConstructorRule for CodeBlockConstructorRule {
    fn name(&self) -> String {
        "```".to_owned()
    }

    fn form(&self) -> crate::core::ast::BlockConstructorForm {
        BlockConstructorForm::Bookend
    }

    fn accept_raw_multiline(&self) -> bool {
        true
    }

    fn apply(
        &self,
        context: &Context,
        main_text: Vec<InlineObjectNode>,
        _: Option<String>,
        multiline_text: Vec<Vec<InlineObjectNode>>,
    ) -> Option<crate::codegen::HtmlNode> {
        let mut source = String::new();

        let mut before = None;
        for token in multiline_text.raw() {
            before = match before {
                None => Some(token),
                Some(
                    t
                    @
                    Token {
                        kind: TokenKind::PunctuationReverseSolidus,
                        ..
                    },
                ) => {
                    match &token.kind {
                        TokenKind::PunctuationsOther(p) if &*p == "```" => {}
                        _ => {
                            source.push_str(&t.text);
                        }
                    }
                    Some(token)
                }
                Some(t) => {
                    source.push_str(&t.text);
                    Some(token)
                }
            };
        }
        if let Some(t) = before {
            source.push_str(&t.text);
        }

        Some(
            HtmlNode::create_tag_builder("div")
                .append(HtmlNode::create_text(escape(&source).to_string()))
                .set_attr("class", "code-block")
                .set_attr(
                    "data-language",
                    main_text
                        .raw()
                        .iter()
                        .map(|t| t.text.clone())
                        .collect::<Vec<_>>()
                        .join(""),
                )
                .build(),
        )
    }
}
