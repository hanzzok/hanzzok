use serde::{
    ser::{self, SerializeMap, SerializeSeq, SerializeStruct},
    Serialize,
};

use crate::{
    value::{HzdataRegexText, HzdataUnitValue},
    Error, HzdataValue,
};

use super::{value_map::HzdataValueSerializeMap, value_seq::HzdataValueSerializeSeq};

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
            HzdataValue::Array(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for e in array {
                    if e == &HzdataValue::Nothing {
                        continue;
                    }
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
            HzdataValue::Object(object) => {
                let mut map = serializer.serialize_map(Some(object.len()))?;
                for (k, v) in object {
                    if v == &HzdataValue::Nothing {
                        continue;
                    }
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            HzdataValue::Nothing => serializer.serialize_none(),
        }
    }
}

pub(crate) struct HzdataValueSerialize;

impl ser::Serializer for HzdataValueSerialize {
    type Ok = HzdataValue;

    type Error = Error<'static>;

    type SerializeSeq = HzdataValueSerializeSeq;

    type SerializeTuple = HzdataValueSerializeSeq;

    type SerializeTupleStruct = HzdataValueSerializeSeq;

    type SerializeTupleVariant = HzdataValueSerializeSeq;

    type SerializeMap = HzdataValueSerializeMap;

    type SerializeStruct = HzdataValueSerializeMap;

    type SerializeStructVariant = HzdataValueSerializeMap;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Boolean(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Integer(v as i64))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Integer(v as i64))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Integer(v as i64))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Integer(v as i64))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Integer(v as i64))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Integer(v as i64))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Integer(v as i64))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Integer(v as i64))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Float(v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Float(v as f64))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::String(v.to_string()))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Nothing)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(HzdataValueSerializeSeq::new())
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(HzdataValueSerializeMap::new())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_map(Some(len))
    }
}
