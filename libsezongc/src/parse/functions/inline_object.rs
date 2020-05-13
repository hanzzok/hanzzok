use super::{decorator, escape, fallback_text, inline_constructor, text, util::*};
use crate::api::InlineObject;
use crate::parse::{ParseResult, Parser};

pub(crate) fn inline_object(parser: &mut Parser<'_>) -> ParseResult<InlineObject> {
    any((escape, text, inline_constructor, decorator, fallback_text))(parser)
}
