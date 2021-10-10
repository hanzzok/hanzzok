use std::{
    iter::Enumerate,
    ops::{RangeFrom, RangeTo},
    vec::IntoIter,
};

use nom::{InputIter, InputLength, InputTake, Needed, Offset, Slice};

use crate::syntax::Token;

#[derive(Clone, Debug)]
pub struct HanzzokParser {
    pub(crate) offset: usize,
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
            offset: self.offset,
        }
    }
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        (
            HanzzokParser {
                tokens: suffix.to_vec(),
                offset: self.offset,
            },
            HanzzokParser {
                tokens: prefix.to_vec(),
                offset: self.offset + count,
            },
        )
    }
}

impl Slice<RangeFrom<usize>> for HanzzokParser {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        HanzzokParser {
            tokens: self.tokens[range.clone()].to_vec(),
            offset: self.offset + range.start,
        }
    }
}

impl Slice<RangeTo<usize>> for HanzzokParser {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        HanzzokParser {
            tokens: self.tokens[range].to_vec(),
            offset: self.offset,
        }
    }
}

impl Offset for HanzzokParser {
    fn offset(&self, second: &Self) -> usize {
        second.offset - self.offset
    }
}
