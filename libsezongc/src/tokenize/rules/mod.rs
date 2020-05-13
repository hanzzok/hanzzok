use crate::core::CodeCharacter;
use crate::tokenize::TokenizerCommand;

pub(crate) trait Rule {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand;
}

mod initial;
mod newline_cr;
mod punctuation;
mod text;
mod whitespace;

pub(crate) use initial::*;
pub(crate) use newline_cr::*;
pub(crate) use punctuation::*;
pub(crate) use text::*;
pub(crate) use whitespace::*;
