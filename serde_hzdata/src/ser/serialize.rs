use std::io::Write;

use serde::Serializer;

use crate::Error;

use super::{serialize_map::HzdataSerializeMap, serialize_seq::HzdataSerializeSeq};

pub(crate) struct HzdataSerialize<W: Write> {
    pub(crate) writer: W,
    pub(crate) pretty: bool,
}

impl<'a, W: Write> Serializer for &'a mut HzdataSerialize<W> {
    type Ok = ();

    type Error = Error<'a>;

    type SerializeSeq = HzdataSerializeSeq<'a, W>;

    type SerializeTuple = HzdataSerializeSeq<'a, W>;

    type SerializeTupleStruct = HzdataSerializeSeq<'a, W>;

    type SerializeTupleVariant = HzdataSerializeSeq<'a, W>;

    type SerializeMap = HzdataSerializeMap<'a, W>;

    type SerializeStruct = HzdataSerializeMap<'a, W>;

    type SerializeStructVariant = HzdataSerializeMap<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        write!(self.writer, "{}", v)?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write!(self.writer, "{}", v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write!(self.writer, "{}", v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        if v.is_infinite() && v.is_sign_positive() {
            write!(self.writer, "inf")?;
        } else if v.is_infinite() && v.is_sign_negative() {
            write!(self.writer, "-inf")?;
        } else if v.is_nan() {
            write!(self.writer, "nan")?;
        } else {
            write!(self.writer, "{}", v)?;
            if (v - v.floor()).abs() < f32::EPSILON {
                write!(self.writer, ".0")?;
            }
        }
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if v.is_infinite() && v.is_sign_positive() {
            write!(self.writer, "inf")?;
        } else if v.is_infinite() && v.is_sign_negative() {
            write!(self.writer, "-inf")?;
        } else if v.is_nan() {
            write!(self.writer, "nan")?;
        } else {
            write!(self.writer, "{}", v)?;
            if (v - v.floor()).abs() < f64::EPSILON {
                write!(self.writer, ".0")?;
            }
        }
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        write!(self.writer, "{:?}", v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        write!(self.writer, "{}", variant)?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(HzdataSerializeSeq::new(self, len))
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

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(HzdataSerializeMap::new(self, len))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(HzdataSerializeMap::new(self, Some(len)))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(HzdataSerializeMap::new(self, Some(len)))
    }
}
