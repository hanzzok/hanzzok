use std::cell::RefCell;
use std::fmt::Display;
use std::path::PathBuf;

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
    // TODO: temporarily open, should be wrapped with compiler api
    pub src: String,
}

impl SourceFile {
    pub(crate) fn new(path: SourceFilePath, src: String) -> SourceFile {
        SourceFile {
            path,
            src: src.clone(),
            id: ID.with(|id| {
                *id.borrow_mut() += 1;
                *id.borrow()
            }),
        }
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

    pub fn parse(&self) -> Vec<crate::api::Spanned<crate::api::Ast>> {
        crate::parse::sezong(nom_locate::LocatedSpan::new(&self.src))
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
