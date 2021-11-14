use v_htmlescape::escape;

use crate::{
    codegen::{Context, HtmlNode},
    core::ast::{DecoratorChainNode, InlineObjectNode, Raw, TextNode},
    syntax::{Token, TokenKind},
};

use super::Walker;

impl Walker<DecoratorChainNode> for Context<'_> {
    fn walk(&mut self, node: DecoratorChainNode) -> Vec<crate::codegen::HtmlNode> {
        let should_be_raw_until = node
            .decorators
            .iter()
            .enumerate()
            .find(|(_, node)| {
                self.compiler
                    .decorator_rules
                    .get(&node.name)
                    .map(|rule| rule.accept_raw_text())
                    .unwrap_or(false)
            })
            .map(|(index, _)| index + 1)
            .unwrap_or(0);
        let should_be_raw = should_be_raw_until > 0;
        let should_be_raw_until = if should_be_raw {
            should_be_raw_until - 1
        } else {
            should_be_raw_until
        };
        let mut nodes = if should_be_raw {
            vec![HtmlNode::create_text(
                escape(
                    &node
                        .main_text
                        .raw()
                        .into_iter()
                        .chain(
                            node.decorators
                                .iter()
                                .take(should_be_raw_until)
                                .flat_map(|node| node.raw()),
                        )
                        .map(|t| t.text.clone())
                        .collect::<String>()
                        .trim_end(),
                )
                .to_string(),
            )]
        } else {
            let node = match *node.main_text {
                InlineObjectNode::Text(text) => {
                    let mut tokens = text.tokens;
                    while let Some((
                        (
                            Token {
                                kind: TokenKind::HorizontalSpace,
                                ..
                            },
                            _,
                        ),
                        elements,
                    )) = tokens.split_last_mut()
                    {
                        if let Some((
                            Token {
                                kind: TokenKind::PunctuationReverseSolidus,
                                ..
                            },
                            _,
                        )) = elements.last()
                        {
                            break;
                        }

                        tokens = elements.to_vec();
                    }
                    InlineObjectNode::Text(TextNode { tokens })
                }
                a @ _ => a,
            };

            self.walk(node)
        };
        let mut is_first = true;
        for decorator in node.decorators.into_iter().skip(should_be_raw_until) {
            let rule = match self.compiler.decorator_rules.get(&decorator.name) {
                Some(rule) => rule,
                None => continue,
            };
            if !is_first && rule.accept_raw_text() {
                continue;
            }
            is_first = false;
            nodes = rule.apply(
                self,
                nodes,
                decorator
                    .params
                    .and_then(|s| serde_hzdata::from_str(&s).ok()),
            );
        }
        nodes
    }
}
