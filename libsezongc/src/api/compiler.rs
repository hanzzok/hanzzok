use super::{Diagnostic, SourceFile, SourceFileId, SourceFilePath};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// The compiler instance
pub struct Compiler {
    diagnostics: Vec<Diagnostic>,
    files: HashMap<SourceFileId, SourceFile>,
}

impl Compiler {
    /// Create a new compiler instance
    pub fn new() -> Compiler {
        Compiler {
            diagnostics: Vec::new(),
            files: HashMap::new(),
        }
    }

    /// Create a new real source file
    pub fn create_real_source<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<&SourceFile> {
        let mut f = File::open(&path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        let file = SourceFile::new(SourceFilePath::Real(path.as_ref().to_path_buf()), s);
        let id = file.id;

        self.files.insert(id, file);
        Ok(self.files.get(&id).expect("Inserted a line before"))
    }

    /// Create a new virtual source file
    pub fn create_virtual_source<P: ToString, S: ToString>(
        &mut self,
        path: P,
        src: S,
    ) -> &SourceFile {
        let file = SourceFile::new(SourceFilePath::Virtual(path.to_string()), src.to_string());
        let id = file.id;

        self.files.insert(id, file);
        self.files.get(&id).expect("Inserted a line before")
    }

    /// Add source file to the compiler
    pub fn add_source_file(&mut self, file: SourceFile) {
        self.files.insert(file.id, file);
    }

    /// Get the source file from compiler with id
    pub fn get_source_file(&self, id: SourceFileId) -> Option<&SourceFile> {
        self.files.get(&id)
    }

    /// Add a new diagnostic
    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Get the diagnostics appears in the compiler
    pub fn diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }
}
