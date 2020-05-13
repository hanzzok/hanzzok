use crate::api::{SourceFile, Span, Token};
use crate::core::CodeCharacter;
use crate::tokenize::rules::*;
use crate::tokenize::{RuleKind, TokenizerCommand};

/// A tokenizer of Sezong
pub struct Tokenizer<'a> {
    pub(crate) source_file: &'a SourceFile,
    characters: Vec<CodeCharacter>,
    offset: usize,
}

impl<'a> Tokenizer<'a> {
    /// Create a new tokenizer based on the source file
    pub fn new(source_file: &'a SourceFile) -> Tokenizer<'a> {
        Tokenizer {
            source_file,
            characters: source_file
                .chars()
                .map(CodeCharacter::new)
                .chain(vec![CodeCharacter::EOF].into_iter())
                .collect(),
            offset: 0,
        }
    }
}

impl<'a> Tokenizer<'a> {
    fn next_token(&mut self) -> Option<Token> {
        let mut result: Option<Token> = None;
        let mut rule_kind = RuleKind::Initial;
        let mut character_cache = Vec::new();
        let mut length = 0;
        'ret: while let Some(character) = self.characters.get(self.offset + length).cloned() {
            // do-while pattern
            'consume: while {
                let command = match rule_kind {
                    RuleKind::Initial => RuleInitial::process(&character, &character_cache),
                    RuleKind::Whitespace => RuleWhitespace::process(&character, &character_cache),
                    RuleKind::NewlineCr => RuleNewlineCr::process(&character, &character_cache),
                    RuleKind::Text => RuleText::process(&character, &character_cache),
                    RuleKind::Punctuation => RulePunctuation::process(&character, &character_cache),
                };
                let to_consume = command.to_consume();
                if to_consume {
                    length += 1;
                    character_cache.push(character.clone());
                }
                match command {
                    TokenizerCommand::Continue(new_rule_kind, _) => {
                        rule_kind = new_rule_kind;
                    }
                    TokenizerCommand::Emit(kind, _) => {
                        result = Some(Token {
                            kind,
                            span: Span::new(self.source_file, self.offset, length),
                        });
                        break 'ret;
                    }
                    TokenizerCommand::Ignore(_) => {
                        break 'consume;
                    }
                }
                !to_consume
            } {}
        }
        self.offset += length;

        result
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
