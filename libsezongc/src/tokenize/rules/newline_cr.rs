use crate::api::TokenKind;
use crate::core::CodeCharacter;
use crate::tokenize::rules::Rule;
use crate::tokenize::TokenizerCommand;

pub(crate) struct RuleNewlineCr;

impl Rule for RuleNewlineCr {
    fn process(character: &CodeCharacter, _: &[CodeCharacter]) -> TokenizerCommand {
        TokenizerCommand::Emit(TokenKind::LineWrap, character.data == '\n')
    }
}
