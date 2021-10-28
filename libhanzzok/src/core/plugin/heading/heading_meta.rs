use crate::codegen::HtmlNodeRef;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HeadingList {
    pub(super) values: Vec<Heading>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Heading {
    pub(super) name: Vec<HtmlNodeRef>,
    pub(super) depth: usize,
}
