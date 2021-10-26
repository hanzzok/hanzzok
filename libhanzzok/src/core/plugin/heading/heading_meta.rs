#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct HeadingList {
    pub(super) values: Vec<Heading>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Heading {
    pub(super) name: String,
    pub(super) depth: usize,
}
