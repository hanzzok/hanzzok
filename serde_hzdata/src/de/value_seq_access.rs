use serde::de;

use crate::{Error, HzdataValue};

pub struct HzdataValueSeqAccess {
    values: Vec<HzdataValue>,
}

impl HzdataValueSeqAccess {
    pub(super) fn new(values: Vec<HzdataValue>) -> Self {
        HzdataValueSeqAccess { values }
    }
}

impl<'de> de::SeqAccess<'de> for HzdataValueSeqAccess {
    type Error = Error<'de>;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.values.is_empty() {
            return Ok(None);
        }

        let value = self.values.remove(0);

        seed.deserialize(value).map(Some)
    }
}
