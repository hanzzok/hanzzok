use std::{collections::VecDeque, iter::once};

use serde_hzdata::HzdataValue;

use crate::{
    api::BlockConstructorRule,
    codegen::{Context, HtmlNode},
    core::{
        ast::{BlockConstructorForm, InlineObjectNode},
        plugin::heading::heading_meta::{Heading, HeadingList},
    },
};

pub struct TocBlockConstructorRule;

impl BlockConstructorRule for TocBlockConstructorRule {
    fn name(&self) -> String {
        "table-of-contents".to_owned()
    }

    fn form(&self) -> crate::core::ast::BlockConstructorForm {
        BlockConstructorForm::Basic
    }

    fn apply(
        &self,
        _context: &mut Context,
        _main_text: Vec<InlineObjectNode>,
        _param: Option<HzdataValue>,
        _: Vec<Vec<InlineObjectNode>>,
    ) -> HtmlNode {
        HtmlNode::create_lazy(|context| {
            let meta: HeadingList = context.load_meta_or_default("heading", "list");

            let mut stack = VecDeque::new();
            stack.push_back((
                Heading {
                    name: Vec::new(),
                    depth: 0,
                },
                HtmlNode::create_tag_builder("ol"),
            ));

            for heading in meta.values {
                if heading.depth > 3 {
                    continue;
                }

                let depth = stack.back().unwrap().0.depth;

                if depth >= heading.depth {
                    loop {
                        let (pop_heading, mut pop) = stack.pop_back().unwrap();
                        let (back_heading, back) = stack.back_mut().unwrap();

                        back.append(HtmlNode::create_tag(
                            "li",
                            &pop_heading
                                .name
                                .into_iter()
                                .map(|node_ref| node_ref.load_from(context).clone())
                                .chain(once(pop.build()))
                                .collect::<Vec<_>>(),
                        ));

                        if back_heading.depth < heading.depth {
                            break;
                        }
                    }
                }

                let depth = stack.back().unwrap().0.depth;

                for depth in depth..(heading.depth - 1) {
                    stack.push_back((
                        Heading {
                            name: Vec::new(),
                            depth,
                        },
                        HtmlNode::create_tag_builder("ol"),
                    ));
                }

                stack.push_back((heading, HtmlNode::create_tag_builder("ol")));
            }

            loop {
                let (pop_heading, mut pop) = stack.pop_back().unwrap();
                let (back_heading, back) = stack.back_mut().unwrap();

                back.append(HtmlNode::create_tag(
                    "li",
                    &pop_heading
                        .name
                        .into_iter()
                        .map(|node_ref| node_ref.load_from(context).clone())
                        .chain(once(pop.build()))
                        .collect::<Vec<_>>(),
                ));

                if back_heading.depth <= 0 {
                    break;
                }
            }

            HtmlNode::create_tag_builder("div")
                .append_all(stack.pop_front().map(|(_, mut builder)| builder.build()))
                .set_attr("class", "table-of-contents")
                .build()
        })
    }
}
