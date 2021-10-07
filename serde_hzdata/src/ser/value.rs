use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::{
    value::{HzdataRegexText, HzdataUnitValue},
    HzdataValue,
};

impl Serialize for HzdataValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            HzdataValue::Boolean(bool) => serializer.serialize_bool(bool.clone()),
            HzdataValue::Integer(integer) => serializer.serialize_i64(integer.clone()),
            HzdataValue::Float(float) => serializer.serialize_f64(float.clone()),
            HzdataValue::String(string) => serializer.serialize_str(string),
            HzdataValue::RegexText(HzdataRegexText { source }) => {
                serializer.serialize_newtype_struct("Regex", source)
            }
            HzdataValue::Unit(HzdataUnitValue { value, unit }) => {
                let mut unit_struct = serializer.serialize_struct("Unit", 2)?;
                unit_struct.serialize_field("value", value)?;
                unit_struct.serialize_field("unit", unit)?;
                unit_struct.end()
            }
        }
    }
}
