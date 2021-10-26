use std::{
    collections::HashMap,
    iter::Enumerate,
    ops::{RangeFrom, RangeTo},
    rc::Rc,
    vec::IntoIter,
};

use nom::{
    combinator::{fail, opt},
    InputIter, InputLength, InputTake, Needed, Offset, Slice,
};
use tokenize::HanzzokTokenizer;

use crate::{
    api::BlockConstructorRule,
    core::ast::BlockConstructorForm,
    syntax::{parse::ParseResult, tokenize, Token, TokenKind},
};

use super::satisfy;

#[derive(Clone)]
pub struct HanzzokParser {
    pub(crate) block_constructors: HashMap<
        BlockConstructorForm,
        Vec<(BlockConstructorNameParser, Rc<dyn BlockConstructorRule>)>,
    >,
    pub(crate) offset: usize,
    pub(crate) tokens: Vec<Token>,
}

impl HanzzokParser {
    pub fn create_tracker(&self) -> HanzzokParserTracker {
        HanzzokParserTracker {
            start_offset: self.offset,
            tokens: self.tokens.clone(),
        }
    }
}

pub struct HanzzokParserTracker {
    start_offset: usize,
    tokens: Vec<Token>,
}

impl HanzzokParserTracker {
    pub fn end(self, parser: &HanzzokParser) -> Vec<Token> {
        self.tokens[..(parser.offset - self.start_offset)].to_vec()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct BlockConstructorNameParser {
    pub(crate) name: String,
    kinds: Vec<TokenKind>,
}

impl BlockConstructorNameParser {
    pub(crate) fn parse(&self, mut p: HanzzokParser) -> ParseResult<Vec<Token>> {
        let mut tokens = Vec::new();

        for kind in &self.kinds {
            let res = satisfy(|token| &token.kind == kind)(p)?;
            p = res.0;
            tokens.push(res.1);
        }

        let (p, v) = opt(satisfy(|t| {
            matches!(
                t.kind,
                TokenKind::PunctuationQuotationMark
                    | TokenKind::PunctuationNumberSign
                    | TokenKind::PunctuationLeftParenthesis
                    | TokenKind::PunctuationRightParenthesis
                    | TokenKind::PunctuationHyphenMinus
                    | TokenKind::PunctuationFullStop
                    | TokenKind::PunctuationReverseSolidus
                    | TokenKind::PunctuationLeftSquareBracket
                    | TokenKind::PunctuationRightSquareBracket
                    | TokenKind::PunctuationLeftCurlyBracket
                    | TokenKind::PunctuationVerticalBar
                    | TokenKind::PunctuationRightCurlyBracket
                    | TokenKind::PunctuationsOther(_)
            )
        }))(p)?;
        if v.is_some() {
            return fail(p);
        }

        Ok((p, tokens))
    }
}

impl HanzzokParser {
    pub fn new(
        tokens: Vec<Token>,
        block_constructors: &HashMap<String, Rc<dyn BlockConstructorRule>>,
    ) -> Self {
        HanzzokParser {
            block_constructors: {
                let mut map = HashMap::new();

                for (name, rule) in block_constructors {
                    let group = map.entry(rule.form()).or_insert(Vec::new());
                    group.push((
                        BlockConstructorNameParser {
                            name: name.clone(),
                            kinds: HanzzokTokenizer::from_source(name.as_ref())
                                .map(|token| token.kind)
                                .collect(),
                        },
                        rule.clone(),
                    ))
                }

                map
            },
            offset: 0,
            tokens,
        }
    }
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
            block_constructors: self.block_constructors.clone(),
        }
    }
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        (
            HanzzokParser {
                tokens: suffix.to_vec(),
                offset: self.offset + count,
                block_constructors: self.block_constructors.clone(),
            },
            HanzzokParser {
                tokens: prefix.to_vec(),
                offset: self.offset,
                block_constructors: self.block_constructors.clone(),
            },
        )
    }
}

impl Slice<RangeFrom<usize>> for HanzzokParser {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        HanzzokParser {
            tokens: self.tokens[range.clone()].to_vec(),
            offset: self.offset + range.start,
            block_constructors: self.block_constructors.clone(),
        }
    }
}

impl Slice<RangeTo<usize>> for HanzzokParser {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        HanzzokParser {
            tokens: self.tokens[range].to_vec(),
            offset: self.offset,
            block_constructors: self.block_constructors.clone(),
        }
    }
}

impl Offset for HanzzokParser {
    fn offset(&self, second: &Self) -> usize {
        second.offset - self.offset
    }
}
