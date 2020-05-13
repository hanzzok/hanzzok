use crate::api::{Compiler, Span};

/// The level of diagnostic
#[derive(Debug, Clone)]
pub enum DiagnosticLevel {
    /// Error level, stops the compiling
    Error,
    /// Warning level, should fix
    Warning,
    /// Note level, additional information to improve the code
    Note,
    /// Help level, additional information to help users
    Help,
}

/// The diagnostic
#[derive(Clone)]
pub struct Diagnostic {
    /// The span area diagnostic occupying
    pub span: Span,
    /// The level of diagnostic
    pub level: DiagnosticLevel,
    /// The message of diagnostic
    pub message: String,
}

impl Diagnostic {
    /// Emit this diagnostic to the compiler
    pub fn emit_to(&self, compiler: &mut Compiler) {
        compiler.add_diagnostic(self.clone());
    }
}
