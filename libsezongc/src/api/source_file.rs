use crate::core::{CodeCharacter, CodeCharacterKind};
use std::cell::RefCell;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::Chars;

thread_local! {
    static ID: RefCell<u64> = RefCell::new(0);
}

#[derive(PartialEq, Debug)]
pub(crate) enum SourceFilePath {
    Real(PathBuf),
    Virtual(String),
}

/// The type for id of source file
pub type SourceFileId = u64;

/// The source file
#[derive(Debug)]
pub struct SourceFile {
    /// The id of source file
    pub id: SourceFileId,
    path: SourceFilePath,
    src: String,
    pub(crate) line_begins: Vec<usize>,
}

impl SourceFile {
    pub(crate) fn new(path: SourceFilePath, src: String) -> SourceFile {
        SourceFile {
            path,
            src: src.clone(),
            line_begins: [
                // virtual line 0
                vec![0],
                // actual lines
                src.chars()
                    .enumerate()
                    .filter(|(_, character)| {
                        CodeCharacter::new(*character).kind == CodeCharacterKind::VerticalSpace
                    })
                    .map(|(offset, _)| offset + 1)
                    .collect(),
                // virtual line 1 + last
                vec![src.chars().count() + 1],
            ]
            .concat(),
            id: ID.with(|id| {
                *id.borrow_mut() += 1;
                *id.borrow()
            }),
        }
    }
}

impl SourceFile {
    pub(crate) fn chars(&self) -> Chars<'_> {
        self.src.chars()
    }
}

impl SourceFile {
    /// The name of file
    pub fn file_name(&self) -> String {
        match &self.path {
            SourceFilePath::Real(path) => {
                path.file_name().map_or("Unnammed".to_string(), |os_str| {
                    os_str.to_string_lossy().to_string()
                })
            }
            SourceFilePath::Virtual(s) => s.clone(),
        }
    }

    /// The path of file
    pub fn path(&self) -> String {
        match &self.path {
            SourceFilePath::Real(path) => path.to_string_lossy().to_string(),
            SourceFilePath::Virtual(s) => s.clone(),
        }
    }

    /// Is the path real or not
    pub fn is_real(&self) -> bool {
        if let SourceFilePath::Real(_) = self.path {
            true
        } else {
            false
        }
    }
}

impl PartialEq for SourceFile {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.path {
            SourceFilePath::Real(path) => write!(f, "SourceFile({})", path.to_string_lossy()),
            SourceFilePath::Virtual(path) => write!(f, "SourceFile(virtual/{})", path),
        }
    }
}
