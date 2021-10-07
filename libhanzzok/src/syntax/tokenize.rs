use core::fmt;
use std::io::Read;
use std::ops::Range;

use logos::{Lexer, Logos};

use crate::core::{LineColumn, Span};

#[derive(Default)]
pub struct TokenizeExtras {
    previous_previous_vertical_space_offset: usize,
    previous_vertical_space_offset: usize,
    current_line: usize,
}

impl TokenizeExtras {
    fn make_vertical_space_span(&self, range: Range<usize>) -> Span {
        Span {
            start: LineColumn {
                line: self.current_line - 1,
                column: range.end - self.previous_previous_vertical_space_offset,
                offset: range.start,
            },
            end: LineColumn {
                line: self.current_line - 1,
                column: range.end - self.previous_previous_vertical_space_offset,
                offset: range.start,
            },
        }
    }
    fn make_span(&self, range: Range<usize>) -> Span {
        Span {
            start: LineColumn {
                line: self.current_line,
                column: range.start - self.previous_vertical_space_offset,
                offset: range.start,
            },
            end: LineColumn {
                line: self.current_line,
                column: range.end - self.previous_vertical_space_offset,
                offset: range.end,
            },
        }
    }
}

#[derive(Clone, Debug, Logos)]
#[logos(extras = TokenizeExtras)]
pub enum TokenKind {
    #[token("#")]
    PunctuationNumberSign,
    #[token(".")]
    PunctuationFullStop,
    #[token("(")]
    PunctuationLeftParenthesis,
    #[token("[")]
    PunctuationLeftSquareBracket,
    #[token("{")]
    PunctuationLeftCurlyBracket,
    #[token(")")]
    PunctuationRightParenthesis,
    #[token("]")]
    PunctuationRightSquareBracket,
    #[token("}")]
    PunctuationRightCurlyBracket,

    #[regex(r"[!#$%&*+,-./:;<=>?@\\\^|~(\[{)\]}]+", |lex| lex.slice().to_owned())]
    PunctuationsOther(String),

    /*
     * "\r\n" : CRLF
     * "\n"   : LINE FEED
     * "\r"   : CARRIAGE RETURN
     */
    #[regex(
        "(\r\n|[\n\r])",
        callback = |lex| {
            lex.extras.current_line += 1;
            lex.extras.previous_previous_vertical_space_offset = lex.extras.previous_vertical_space_offset;
            lex.extras.previous_vertical_space_offset = lex.span().end;
        }
    )]
    VerticalSpace,

    #[regex("[ ]")]
    HorizontalSpace,

    #[regex(r"[^\n\r !#$%&*+,-./:;<=>?@\\\^|~(\[{)\]}]+", |lex| lex.slice().to_owned())]
    Word(String),

    #[error]
    Error,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at {}", self.kind, self.span)
    }
}

pub struct HanzzokTokenizer<'a> {
    lexer: Lexer<'a, TokenKind>,
}

impl<'a> HanzzokTokenizer<'a> {
    pub fn from_source(source: &'a str) -> Self {
        HanzzokTokenizer {
            lexer: TokenKind::lexer(source),
        }
    }
}

impl<'a> Iterator for HanzzokTokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let span = if let TokenKind::VerticalSpace = &kind {
            self.lexer
                .extras
                .make_vertical_space_span(self.lexer.span())
        } else {
            self.lexer.extras.make_span(self.lexer.span())
        };
        Some(Token { kind, span })
    }
}