use core::fmt;

#[derive(Clone, Debug, Eq, Ord, serde::Serialize, serde::Deserialize)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl fmt::Display for LineColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L{}:{}", self.line, self.column)
    }
}

impl PartialEq for LineColumn {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl PartialOrd for LineColumn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}
