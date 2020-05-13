use crate::api::{Ast, SourceFile, Span, Spanned, Token};
use crate::parse::functions::sezong;
use crate::Tokenizer;

/// The parser
pub struct Parser<'a> {
    pub(crate) source_file: &'a SourceFile,
    pub(crate) cursor: usize,
    pub(crate) tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    /// Create a new parser based on the tokens
    pub fn from_tokens<I>(source_file: &'a SourceFile, tokens: I) -> Parser<'a>
    where
        I: Iterator<Item = Token>,
    {
        Parser {
            source_file,
            cursor: 0,
            tokens: tokens.collect(),
        }
    }
    /// Create a new parser based on the tokenizer
    pub fn from_tokenizer(tokenizer: Tokenizer<'a>) -> Parser<'a> {
        Parser::from_tokens(tokenizer.source_file, tokenizer)
    }

    /// Parse sezong document
    pub fn parse(&mut self) -> ParseResult<Vec<Ast>> {
        sezong(self)
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn last_used(&self) -> Option<&Token> {
        if self.has_next() {
            Some(&self.tokens[self.cursor])
        } else {
            self.tokens.last()
        }
    }
    pub(crate) fn span_last_used(&self) -> Span {
        self.last_used()
            .map(Spanned::span)
            .unwrap_or_else(|| Span::zero(self.source_file.id))
    }
    pub(crate) fn has_next(&self) -> bool {
        self.cursor < self.tokens.len()
    }
}

/// The result of parsing
pub type ParseResult<T> = Result<T, ParseError>;

pub(crate) trait Ignorable<T, E> {
    fn ignored(self) -> Result<Option<T>, E>;
}

impl<T> Ignorable<T, ParseError> for ParseResult<T> {
    fn ignored(self) -> Result<Option<T>, ParseError> {
        match self {
            Err(ParseError::Unmatched(_)) => Ok(None),
            Ok(val) => Ok(Some(val)),
            Err(val) => val.into(),
        }
    }
}

/// The error trait of parsing
#[derive(Debug)]
pub enum ParseError {
    /// When the parser encountered an unexpected token
    UnexpectedToken(Token),
    /// When the parser encountered unexpected EOF
    UnexpectedEof(Span),
    /// When the parser encountered unmatched token, should be ignored
    Unmatched(Span),
    /// When the parsing was ended but there's left tokens
    Unused(Span),
}

impl Spanned for ParseError {
    fn span(&self) -> Span {
        match self {
            ParseError::UnexpectedToken(token, ..) => token.span(),
            ParseError::UnexpectedEof(span, ..) => span.clone(),
            ParseError::Unmatched(span, ..) => span.clone(),
            ParseError::Unused(span, ..) => span.clone(),
        }
    }
}

impl<T> Into<ParseResult<T>> for ParseError {
    fn into(self) -> ParseResult<T> {
        Err(self)
    }
}
