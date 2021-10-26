use std::{collections::HashMap, hash::Hash};

use crate::Unit;

#[derive(Clone, Debug, PartialEq)]
pub enum HzdataValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    RegexText(HzdataRegexText),
    Unit(HzdataUnitValue),
    Array(Vec<HzdataValue>),
    Object(HashMap<String, HzdataValue>),
    // Just for the serde
    Nothing,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct HzdataRegexText {
    pub(crate) source: String,
}

impl HzdataRegexText {
    pub fn as_regex(&self) -> Result<regex::Regex, regex::Error> {
        regex::Regex::new(&self.source)
    }
}

#[derive(Clone, Debug)]
pub struct HzdataUnitValue {
    pub value: f64,
    pub unit: Unit,
}

impl Hash for HzdataUnitValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.value as u64).hash(state);
        self.unit.hash(state);
    }
}

impl PartialEq for HzdataUnitValue {
    fn eq(&self, other: &Self) -> bool {
        ((self.value.is_nan() && other.value.is_nan()) || (self.value == other.value))
            && self.unit == other.unit
    }
}

impl Eq for HzdataUnitValue {}
