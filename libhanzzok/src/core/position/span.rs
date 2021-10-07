use core::fmt;

use super::LineColumn;

#[derive(Clone, Debug)]
pub struct Span {
    pub start: LineColumn,
    pub end: LineColumn,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}..{}", self.start, self.end)
    }
}
