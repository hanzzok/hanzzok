#[derive(Debug)]
pub(crate) enum RuleKind {
    Initial,
    NewlineCr,
    Whitespace,
    Text,
    Punctuation,
}
