use serde::de;

use crate::{
    de::{MapAccess, SeqAccess},
    Error,
};

use super::parse::{
    parse_bool, parse_identifier, parse_number, parse_string, skip_whitespaces, Number,
};

pub struct HzdataDeserializer<'de> {
    pub(crate) source: &'de [u8],
}

impl<'a, 'de: 'a> HzdataDeserializer<'de> {
    fn skip_whitespaces(&'a mut self) -> Result<(), Error<'de>> {
        let (s, _) = skip_whitespaces(self.source)?;
        self.source = s;
        Ok(())
    }

    fn peek_bool(&'a mut self) -> Result<bool, Error<'de>> {
        let (source, val) = parse_bool(self.source)?;
        self.source = source;
        Ok(val)
    }

    fn peek_number(&'a mut self) -> Result<Number, Error<'de>> {
        let (source, val) = parse_number(self.source)?;
        self.source = source;
        Ok(val)
    }

    fn peek_string(&'a mut self) -> Result<String, Error<'de>> {
        let (source, val) = parse_string(self.source)?;
        self.source = source;
        Ok(val)
    }

    fn deserialize_number<V>(&'a mut self, visitor: V) -> Result<V::Value, Error<'de>>
    where
        V: de::Visitor<'de>,
    {
        match self.peek_number()? {
            Number::I64(val) => visitor.visit_i64(val),
            Number::F64(val) => visitor.visit_f64(val),
        }
    }
}

macro_rules! deserialize_number {
    ($name:ident) => {
        fn $name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            self.deserialize_number(visitor)
        }
    };
}

impl<'a, 'de: 'a> de::Deserializer<'de> for &'a mut HzdataDeserializer<'de> {
    type Error = Error<'de>;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.skip_whitespaces()?;
        if let Ok(bool) = self.peek_bool() {
            visitor.visit_bool(bool)
        } else if let Ok(number) = self.peek_number() {
            match number {
                Number::I64(val) => visitor.visit_i64(val),
                Number::F64(val) => visitor.visit_f64(val),
            }
        } else if let Ok(string) = self.peek_string() {
            visitor.visit_string(string)
        } else if self.source[0] == b'[' {
            self.deserialize_seq(visitor)
        } else if self.source[0] == b'{' {
            self.deserialize_map(visitor)
        } else {
            dbg!(self.source);
            todo!()
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(self.peek_bool()?)
    }

    deserialize_number!(deserialize_i8);
    deserialize_number!(deserialize_i16);
    deserialize_number!(deserialize_i32);
    deserialize_number!(deserialize_i64);

    deserialize_number!(deserialize_u8);
    deserialize_number!(deserialize_u16);
    deserialize_number!(deserialize_u32);
    deserialize_number!(deserialize_u64);

    deserialize_number!(deserialize_f32);
    deserialize_number!(deserialize_f64);

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_str(&self.peek_string()?)
    }

    fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.peek_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let seq_access = SeqAccess::new(&mut self);
        visitor.visit_seq(seq_access)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let map_access = MapAccess::new(self);
        visitor.visit_map(map_access)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let (source, val) = parse_identifier(self.source)?;
        self.source = source;
        visitor.visit_str(&val)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}
