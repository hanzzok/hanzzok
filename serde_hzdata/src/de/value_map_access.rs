use std::collections::HashMap;

use serde::de;

use crate::{Error, HzdataValue};

pub struct HzdataValueMapAccess {
    values: Vec<(String, HzdataValue)>,
}

impl HzdataValueMapAccess {
    pub(super) fn new(values: HashMap<String, HzdataValue>) -> Self {
        HzdataValueMapAccess {
            values: values.into_iter().collect(),
        }
    }
}

impl<'de> de::MapAccess<'de> for HzdataValueMapAccess {
    type Error = Error<'de>;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.values.is_empty() {
            return Ok(None);
        }

        struct SeedDeserializer {
            key: String,
        }

        impl<'de> de::Deserializer<'de> for SeedDeserializer {
            type Error = Error<'static>;

            fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>,
            {
                visitor.visit_string(self.key)
            }

            serde::forward_to_deserialize_any! {
                bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
            }
        }

        seed.deserialize(SeedDeserializer {
            key: self.values[0].0.clone(),
        })
        .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let value = self.values.remove(0).1;
        seed.deserialize(value)
    }
}
