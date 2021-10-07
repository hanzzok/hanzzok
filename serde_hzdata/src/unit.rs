use serde::Serialize;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Unit {
    Length(LengthUnit),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum LengthUnit {
    #[serde(rename = "px")]
    Pixel,
}
