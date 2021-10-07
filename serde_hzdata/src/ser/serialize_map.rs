use std::io::Write;

use crate::Error;

use super::serialize::HzdataSerialize;

#[derive(PartialEq)]
enum State {
    First,
    Rest,
}

pub(crate) struct HzdataSerializeMap<'a, W: Write> {
    ser: &'a mut HzdataSerialize<W>,
    len: Option<usize>,
    state: State,
}

impl<'a, W: Write> HzdataSerializeMap<'a, W> {
    pub(crate) fn new(ser: &'a mut HzdataSerialize<W>, len: Option<usize>) -> Self {
        HzdataSerializeMap {
            ser,
            len,
            state: State::First,
        }
    }
}

impl<'a, W: Write> serde::ser::SerializeMap for HzdataSerializeMap<'a, W> {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if self.state == State::First {
            write!(self.ser.writer, "{{")?;
            self.state = State::Rest
        } else {
            write!(self.ser.writer, ",")?;
        }
        key.serialize(&mut *self.ser)?;
        write!(self.ser.writer, "=")?;

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write!(self.ser.writer, "}}")?;
        Ok(())
    }
}

impl<'a, W: Write> serde::ser::SerializeStruct for HzdataSerializeMap<'a, W> {
    type Ok = ();

    type Error = Error;

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

impl<'a, W: Write> serde::ser::SerializeStructVariant for HzdataSerializeMap<'a, W> {
    type Ok = ();

    type Error = Error;

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
