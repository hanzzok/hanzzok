use crate::api::TokenKind;
use crate::core::{CodeCharacter, CodeCharacterKind};
use crate::tokenize::rules::Rule;
use crate::tokenize::{RuleKind, TokenizerCommand};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref PUNCTUATION_MAP: HashMap<&'static str, TokenKind> = {
        let mut map = HashMap::new();
        map.insert("!", TokenKind::PunctuationExclamationMark);
        map.insert(".", TokenKind::PunctuationFullStop);
        map.insert("\\", TokenKind::PunctuationReverseSolidus);
        map.insert("|", TokenKind::PunctuationVerticalLine);
        map.insert("(", TokenKind::PunctuationLeftParenthesis);
        map.insert(")", TokenKind::PunctuationRightParenthesis);
        map.insert("[", TokenKind::PunctuationLeftSquareBracket);
        map.insert("]", TokenKind::PunctuationRightSquareBracket);
        // map.insert("{", TokenKind::PunctuationLeftCurlyBracket);
        // map.insert("}", TokenKind::PunctuationRightCurlyBracket);
        map
    };
}

pub(crate) struct RulePunctuation;

impl Rule for RulePunctuation {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand {
        match character.kind {
            CodeCharacterKind::Punctuation if !characters.is_empty() => {
                let punctuation_joined: String = [characters, &[character.clone()]]
                    .concat()
                    .iter()
                    .map(|character| character.data)
                    .collect();
                for (punctuation, token_kind) in PUNCTUATION_MAP.iter() {
                    if punctuation.starts_with(&punctuation_joined) {
                        return TokenizerCommand::Continue(RuleKind::Punctuation, true);
                    }
                    if &punctuation_joined == punctuation {
                        return TokenizerCommand::Emit(token_kind.clone(), true);
                    }
                }
                TokenizerCommand::Emit(
                    PUNCTUATION_MAP
                        .get(
                            &characters
                                .iter()
                                .map(|character| character.data)
                                .collect::<String>()
                                .as_str(),
                        )
                        .unwrap_or(&TokenKind::PunctuationAscii)
                        .clone(),
                    false,
                )
            }
            _ => TokenizerCommand::Emit(
                PUNCTUATION_MAP
                    .get(
                        &characters
                            .iter()
                            .map(|character| character.data)
                            .collect::<String>()
                            .as_str(),
                    )
                    .unwrap_or(&TokenKind::PunctuationAscii)
                    .clone(),
                false,
            ),
        }
    }
}
