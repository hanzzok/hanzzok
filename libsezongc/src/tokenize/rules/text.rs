use crate::api::TokenKind;
use crate::core::{CodeCharacter, CodeCharacterKind};
use crate::tokenize::rules::Rule;
use crate::tokenize::{RuleKind, TokenizerCommand};

pub(crate) struct RuleText;

impl Rule for RuleText {
    fn process(character: &CodeCharacter, _: &[CodeCharacter]) -> TokenizerCommand {
        match character.kind {
            CodeCharacterKind::Text => TokenizerCommand::Continue(RuleKind::Text, true),
            _ => TokenizerCommand::Emit(TokenKind::Text, false),
        }
    }
}
