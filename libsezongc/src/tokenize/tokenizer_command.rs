use crate::api::TokenKind;
use crate::tokenize::rule_kind::RuleKind;

pub(crate) type ToConsume = bool;

#[derive(Debug)]
pub(crate) enum TokenizerCommand {
    Continue(RuleKind, ToConsume),
    Emit(TokenKind, ToConsume),
    Ignore(ToConsume),
}

impl TokenizerCommand {
    pub(crate) fn to_consume(&self) -> ToConsume {
        match self {
            TokenizerCommand::Continue(_, it) => *it,
            TokenizerCommand::Emit(_, it) => *it,
            TokenizerCommand::Ignore(it) => *it,
        }
    }
}
