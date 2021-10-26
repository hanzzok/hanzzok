use crate::{Error, HzdataValue};

use super::value::HzdataValueSerialize;

pub struct HzdataValueSerializeSeq {
    values: Vec<HzdataValue>,
}

impl HzdataValueSerializeSeq {
    pub(crate) fn new() -> Self {
        HzdataValueSerializeSeq { values: Vec::new() }
    }
}

impl serde::ser::SerializeSeq for HzdataValueSerializeSeq {
    type Ok = HzdataValue;

    type Error = Error<'static>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.values.push(value.serialize(HzdataValueSerialize)?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Array(self.values))
    }
}

impl serde::ser::SerializeTuple for HzdataValueSerializeSeq {
    type Ok = HzdataValue;

    type Error = Error<'static>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleStruct for HzdataValueSerializeSeq {
    type Ok = HzdataValue;

    type Error = Error<'static>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        serde::ser::SerializeTuple::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeTuple::end(self)
    }
}

impl serde::ser::SerializeTupleVariant for HzdataValueSerializeSeq {
    type Ok = HzdataValue;

    type Error = Error<'static>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        serde::ser::SerializeTupleStruct::serialize_field(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeTupleStruct::end(self)
    }
}
