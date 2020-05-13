use crate::api::TokenKind;
use crate::core::{CodeCharacter, CodeCharacterKind};
use crate::tokenize::rules::Rule;
use crate::tokenize::{RuleKind, TokenizerCommand};

pub(crate) struct RuleWhitespace;

impl Rule for RuleWhitespace {
    fn process(character: &CodeCharacter, _: &[CodeCharacter]) -> TokenizerCommand {
        match character.kind {
            CodeCharacterKind::HorizontalSpace => {
                TokenizerCommand::Continue(RuleKind::Whitespace, true)
            }
            _ => TokenizerCommand::Emit(TokenKind::Whitespace, false),
        }
    }
}
