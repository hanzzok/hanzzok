use crate::api::{
    Compiler, Diagnostic, DiagnosticLevel, LineColumn, SourceFile, SourceFileId, Spanned,
};
use std::cmp::{max, min};
use std::fmt::Display;

/// The span information
#[derive(Clone, Debug)]
pub struct Span {
    source_file: SourceFileId,
    /// The start point of span
    pub start: LineColumn,
    /// The end point of span
    pub end: LineColumn,
}

fn create_line_column(source_file: &SourceFile, offset: usize) -> LineColumn {
    let (line, column) = match source_file.line_begins.binary_search(&offset) {
        Ok(line) => (line + 1, 0),
        Err(line) => (
            line,
            offset - source_file.line_begins.get(line - 1).unwrap_or(&0),
        ),
    };

    LineColumn {
        offset,
        line,
        column,
    }
}
impl Span {
    pub(crate) fn zero(source_file: SourceFileId) -> Span {
        Span {
            source_file,
            start: LineColumn::ZERO,
            end: LineColumn::ZERO,
        }
    }
    pub(crate) fn new(source_file: &SourceFile, start: usize, length: usize) -> Span {
        Span {
            source_file: source_file.id,
            start: create_line_column(source_file, start),
            end: create_line_column(source_file, start + length),
        }
    }
    pub(crate) fn from_sequence<T, I>(iterator: I) -> Option<Span>
    where
        T: Spanned,
        I: Iterator<Item = T>,
    {
        let vec: Vec<_> = iterator.collect();
        if let Some(first) = vec.first() {
            let last = vec.last().expect("If first set, last set");
            first.span().joined(&last.span())
        } else {
            None
        }
    }
}

impl Span {
    /// The length of the span
    pub fn len(&self) -> usize {
        self.end.offset - self.start.offset
    }

    /// Is the span empty or not
    pub fn is_empty(&self) -> bool {
        self.end.offset == self.start.offset
    }

    /// Get the source file instance where the span is on from compiler
    pub fn source_file<'a>(&self, compiler: &'a Compiler) -> Option<&'a SourceFile> {
        compiler.get_source_file(self.source_file)
    }

    /// Join two spans, `None` if the source files of the spans are different
    pub fn joined(&self, other: &Span) -> Option<Span> {
        if self.source_file != other.source_file {
            return None;
        }

        let start = min(&self.start, &other.start).clone();
        let end = max(&self.end, &other.end).clone();

        Some(Span {
            source_file: self.source_file,
            start,
            end,
        })
    }

    /// Get the source text from the session
    pub fn source_text(&self, compiler: &Compiler) -> Option<String> {
        self.source_file(compiler).map(|source_file| {
            source_file
                .chars()
                .skip(self.start.offset)
                .take(self.len())
                .collect::<String>()
        })
    }

    /// Create a Error level Diagnostic with supplied message occupying this Span
    pub fn diagnostic_error<T: ToString>(&self, message: T) -> Diagnostic {
        Diagnostic {
            span: self.clone(),
            level: DiagnosticLevel::Error,
            message: message.to_string(),
        }
    }

    /// Create a Warning level Diagnostic with supplied message occupying this Span
    pub fn diagnostic_warning<T: ToString>(&self, message: T) -> Diagnostic {
        Diagnostic {
            span: self.clone(),
            level: DiagnosticLevel::Warning,
            message: message.to_string(),
        }
    }

    /// Create a Note level Diagnostic with supplied message occupying this Span
    pub fn diagnostic_note<T: ToString>(&self, message: T) -> Diagnostic {
        Diagnostic {
            span: self.clone(),
            level: DiagnosticLevel::Note,
            message: message.to_string(),
        }
    }

    /// Create a Help level Diagnostic with supplied message occupying this Span
    pub fn diagnostic_help<T: ToString>(&self, message: T) -> Diagnostic {
        Diagnostic {
            span: self.clone(),
            level: DiagnosticLevel::Help,
            message: message.to_string(),
        }
    }
}

impl Spanned for Span {
    fn span(&self) -> Span {
        self.clone()
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
