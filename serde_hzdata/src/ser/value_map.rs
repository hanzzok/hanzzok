use std::{collections::HashMap, io::Write};

use crate::{Error, HzdataValue};

use super::{serialize::HzdataSerialize, value::HzdataValueSerialize};

pub(crate) struct HzdataValueSerializeMap {
    last_key: Option<String>,
    values: HashMap<String, HzdataValue>,
}

impl HzdataValueSerializeMap {
    pub(crate) fn new() -> Self {
        HzdataValueSerializeMap {
            last_key: None,
            values: HashMap::new(),
        }
    }
}

impl serde::ser::SerializeMap for HzdataValueSerializeMap {
    type Ok = HzdataValue;

    type Error = Error<'static>;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.last_key = Some(match key.serialize(HzdataValueSerialize)? {
            HzdataValue::String(s) => s,
            _ => todo!(),
        });

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.values.insert(
            self.last_key.clone().unwrap(),
            value.serialize(HzdataValueSerialize)?,
        );
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(HzdataValue::Object(self.values))
    }
}

impl serde::ser::SerializeStruct for HzdataValueSerializeMap {
    type Ok = HzdataValue;

    type Error = Error<'static>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        serde::ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeMap::end(self)
    }
}

impl serde::ser::SerializeStructVariant for HzdataValueSerializeMap {
    type Ok = HzdataValue;

    type Error = Error<'static>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        serde::ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeStruct::end(self)
    }
}
