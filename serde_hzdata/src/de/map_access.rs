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

pub struct MapAccess<'a, 'de: 'a> {
    deserializer: &'a mut HzdataDeserializer<'de>,
    state: State,
}

impl<'a, 'de: 'a> MapAccess<'a, 'de> {
    pub(super) fn new(deserializer: &'a mut HzdataDeserializer<'de>) -> Self {
        MapAccess {
            deserializer,
            state: State::First,
        }
    }
}

impl<'a, 'de: 'a> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error<'de>;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        let s = self.deserializer.source;
        let (s, _) = skip_whitespaces(s)?;
        let (s, close) = cond(self.state == State::Rest, opt(tag(b"}")))(s)?;

        if close.flatten().is_some() {
            self.deserializer.source = s;
            self.state = State::Done;
            return Ok(None);
        }

        let (s, _) = (if self.state == State::First {
            opt(tag(b"{"))(s)
        } else {
            map(tag(b","), Some)(s)
        })?;
        let (s, _) = skip_whitespaces(s)?;
        if self.state == State::First {
            let (s, close) = opt(tag(b"}"))(s)?;
            if close.is_some() {
                self.deserializer.source = s;
                self.state = State::Done;
                return Ok(None);
            }
        }
        self.state = State::Rest;

        self.deserializer.source = s;

        seed.deserialize(&mut *self.deserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let s = self.deserializer.source;
        let (s, _) = skip_whitespaces(s)?;
        let (s, _) = tag(b"=")(s)?;
        let (s, _) = skip_whitespaces(s)?;

        self.deserializer.source = s;
        seed.deserialize(&mut *self.deserializer)
    }
}
