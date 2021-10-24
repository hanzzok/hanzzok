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

    type Error = Error<'a>;

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
        match key.serialize(&mut *self.ser) {
            Ok(_) => {}
            Err(Error::Io(io)) => {
                return Err(Error::Io(io));
            }
            Err(Error::Custom(message)) => {
                return Err(Error::Custom(message));
            }
            Err(Error::Nom(_)) => {
                panic!("Serialize must not throw nom error.")
            }
        }
        write!(self.ser.writer, "=")?;

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        match value.serialize(&mut *self.ser) {
            Ok(_) => {}
            Err(Error::Io(io)) => {
                return Err(Error::Io(io));
            }
            Err(Error::Custom(message)) => {
                return Err(Error::Custom(message));
            }
            Err(Error::Nom(_)) => {
                panic!("Serialize must not throw nom error.")
            }
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write!(self.ser.writer, "}}")?;
        Ok(())
    }
}

impl<'a, W: Write> serde::ser::SerializeStruct for HzdataSerializeMap<'a, W> {
    type Ok = ();

    type Error = Error<'a>;

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

    type Error = Error<'a>;

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
