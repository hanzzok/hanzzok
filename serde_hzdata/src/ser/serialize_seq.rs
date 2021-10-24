use std::{
    io::Write,
    ops::{Deref, DerefMut},
};

use crate::Error;

use super::serialize::HzdataSerialize;

#[derive(PartialEq)]
pub enum State {
    First,
    Rest,
}

pub struct HzdataSerializeSeq<'a, W: Write> {
    ser: &'a mut HzdataSerialize<W>,
    len: Option<usize>,
    state: State,
}

impl<'a, W: Write> HzdataSerializeSeq<'a, W> {
    pub(crate) fn new(ser: &'a mut HzdataSerialize<W>, len: Option<usize>) -> Self {
        HzdataSerializeSeq {
            ser,
            len,
            state: State::First,
        }
    }
}

impl<'a, W: Write> serde::ser::SerializeSeq for HzdataSerializeSeq<'a, W> {
    type Ok = ();

    type Error = Error<'a>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if self.state == State::First {
            write!(self.ser.writer, "[")?;
            self.state = State::Rest
        } else {
            write!(self.ser.writer, ",")?;
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
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write!(self.ser.writer, "]")?;
        Ok(())
    }
}

impl<'a, W: Write> serde::ser::SerializeTuple for HzdataSerializeSeq<'a, W> {
    type Ok = ();

    type Error = Error<'a>;

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

impl<'a, W: Write> serde::ser::SerializeTupleStruct for HzdataSerializeSeq<'a, W> {
    type Ok = ();

    type Error = Error<'a>;

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

impl<'a, W: Write> serde::ser::SerializeTupleVariant for HzdataSerializeSeq<'a, W> {
    type Ok = ();

    type Error = Error<'a>;

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
