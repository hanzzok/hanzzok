use nom::{
    bytes::complete::tag,
    combinator::{cond, map, opt},
};
use serde::de;

use crate::Error;

use super::{deserialize::HzdataDeserializer, parse::skip_whitespaces};

#[derive(PartialEq)]
enum State {
    First,
    Rest,
    Done,
}

pub struct SeqAccess<'a, 'de: 'a> {
    deserializer: &'a mut HzdataDeserializer<'de>,
    state: State,
}

impl<'a, 'de: 'a> SeqAccess<'a, 'de> {
    pub(super) fn new(deserializer: &'a mut HzdataDeserializer<'de>) -> Self {
        SeqAccess {
            deserializer,
            state: State::First,
        }
    }
}

impl<'a, 'de: 'a> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error<'de>;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.state == State::Done {
            panic!("WTF");
        }
        let s = self.deserializer.source;
        let (s, _) = skip_whitespaces(s)?;
        let (s, close) = cond(self.state == State::Rest, opt(tag(b"]")))(s)?;

        if close.flatten().is_some() {
            self.deserializer.source = s;
            self.state = State::Done;
            return Ok(None);
        }

        let (s, _) = (if self.state == State::First {
            opt(tag(b"["))(s)
        } else {
            map(tag(b","), Some)(s)
        })?;
        self.state = State::Rest;

        let (s, _) = skip_whitespaces(s)?;

        self.deserializer.source = s;

        seed.deserialize(&mut *self.deserializer).map(Some)
    }
}
