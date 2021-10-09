use std::{iter::Enumerate, ops::RangeFrom, vec::IntoIter};

use nom::{InputIter, InputLength, InputTake, Needed, Slice};

use crate::syntax::Token;

#[derive(Clone, Debug)]
pub struct HanzzokParser {
    pub(crate) block_constructor_names: Vec<String>,
    pub(crate) tokens: Vec<Token>,
}

impl InputLength for HanzzokParser {
    #[inline]
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl<'a> InputIter for HanzzokParser {
    type Item = Token;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = IntoIter<Token>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }
    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.tokens.clone().into_iter()
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.tokens.iter().position(|b| predicate(b.clone()))
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
            Err(Needed::new(count - self.tokens.len()))
        }
    }
}

impl InputTake for HanzzokParser {
    #[inline]
    fn take(&self, count: usize) -> Self {
        HanzzokParser {
            tokens: self.tokens[0..count].to_vec(),
            block_constructor_names: self.block_constructor_names.clone(),
        }
    }
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        (
            HanzzokParser {
                tokens: suffix.to_vec(),
                block_constructor_names: self.block_constructor_names.clone(),
            },
            HanzzokParser {
                tokens: prefix.to_vec(),
                block_constructor_names: self.block_constructor_names.clone(),
            },
        )
    }
}

impl Slice<RangeFrom<usize>> for HanzzokParser {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        HanzzokParser {
            tokens: self.tokens[range].to_vec(),
            block_constructor_names: self.block_constructor_names.clone(),
        }
    }
}
