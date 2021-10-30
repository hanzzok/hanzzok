use core::fmt;

use super::{LineColumn, Spanned};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Span {
    pub start: LineColumn,
    pub end: LineColumn,
}

impl Span {
    pub fn joined(&self, other: &impl Spanned) -> Span {
        let other = other.span();
        if self.end < other.start {
            Span {
                start: self.start.clone(),
                end: other.end.clone(),
            }
        } else {
            Span {
                start: other.start.clone(),
                end: self.end.clone(),
            }
        }
    }
    pub fn joined_opt(&self, other: Option<&impl Spanned>) -> Span {
        other
            .map(|o| o.span().joined(self))
            .unwrap_or_else(|| self.clone())
    }
}

impl Spanned for Span {
    fn span(&self) -> Span {
        self.clone()
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}
