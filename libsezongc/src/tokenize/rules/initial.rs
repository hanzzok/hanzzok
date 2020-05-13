use crate::api::TokenKind;
use crate::core::{CodeCharacter, CodeCharacterKind};
use crate::tokenize::rules::Rule;
use crate::tokenize::{RuleKind, TokenizerCommand};

pub(crate) struct RuleInitial;

impl Rule for RuleInitial {
    fn process(character: &CodeCharacter, _: &[CodeCharacter]) -> TokenizerCommand {
        match character.kind {
            CodeCharacterKind::HorizontalSpace => {
                TokenizerCommand::Continue(RuleKind::Whitespace, true)
            }
            CodeCharacterKind::VerticalSpace => {
                if character.data == '\r' {
                    TokenizerCommand::Continue(RuleKind::NewlineCr, true)
                } else {
                    TokenizerCommand::Emit(TokenKind::LineWrap, true)
                }
            }
            CodeCharacterKind::Punctuation => {
                TokenizerCommand::Continue(RuleKind::Punctuation, true)
            }
            CodeCharacterKind::Text => TokenizerCommand::Continue(RuleKind::Text, true),
            CodeCharacterKind::EOF => TokenizerCommand::Ignore(true),
        }
    }
}
