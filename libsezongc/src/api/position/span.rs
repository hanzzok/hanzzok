use crate::api::{Diagnostic, DiagnosticLevel, LineColumn};
use std::fmt::Display;

/// The span information
#[derive(Clone, Debug)]
pub struct Span {
    /// The start point of span
    pub start: LineColumn,
    /// The end point of span
    pub end: LineColumn,
}

impl Span {
    /// Create a span with line column information supplied
    pub fn new(start: LineColumn, end: LineColumn) -> Self {
        Span { start, end }
    }

    /// The length of the span
    pub fn len(&self) -> usize {
        self.end.offset - self.start.offset
    }

    /// Is the span empty or not
    pub fn is_empty(&self) -> bool {
        self.end.offset == self.start.offset
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

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
