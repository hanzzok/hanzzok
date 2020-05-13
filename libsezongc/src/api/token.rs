use crate::api::{Span, Spanned};
use std::fmt::Display;

/// The kind of token
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// Any punctuation is not `!`, `.`, `(`, `)`, `[`, `]`, `{`, and `}`
    PunctuationAscii,
    /// Punctuation `!`
    PunctuationExclamationMark,
    /// Punctuation `.`
    PunctuationFullStop,
    /// Punctuation `\`
    PunctuationReverseSolidus,
    /// Punctuation `|`
    PunctuationVerticalLine,
    /// Punctuation `(`
    PunctuationLeftParenthesis,
    /// Punctuation `)`
    PunctuationRightParenthesis,
    /// Punctuation `[`
    PunctuationLeftSquareBracket,
    /// Punctuation `]`
    PunctuationRightSquareBracket,
    /// Punctuation `{`
    PunctuationLeftCurlyBracket,
    /// Punctuation `}`
    PunctuationRightCurlyBracket,
    /// Line wrap
    LineWrap,
    /// Whitespaces
    Whitespace,
    /// Text
    Text,
}

/// The token
#[derive(Debug, Clone)]
pub struct Token {
    /// The kind of this token
    pub kind: TokenKind,
    /// The span of this token
    pub span: Span,
}

impl Token {
    /// The length of token
    pub fn len(&self) -> usize {
        self.span.len()
    }
    /// Is the token empty or not
    pub fn is_empty(&self) -> bool {
        self.span.is_empty()
    }
}

impl Spanned for Token {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Token/{:?} at {}]", self.kind, self.span)
    }
}
