use crate::parse::{ParseError, ParseResult, Parser};

pub(crate) fn take_while<F, T: std::fmt::Debug>(
    mut function: F,
) -> impl FnMut(&mut Parser<'_>) -> ParseResult<Vec<T>>
where
    F: FnMut(&mut Parser<'_>) -> ParseResult<T>,
{
    move |parser| {
        let mut cursor = parser.cursor;
        let mut result = Vec::new();
        while parser.has_next() {
            match function(parser) {
                Ok(value) => {
                    cursor = parser.cursor;
                    result.push(value);
                }
                Err(ParseError::Unmatched(_)) => {
                    parser.cursor = cursor;
                    break;
                }
                Err(error) => return error.into(),
            }
        }
        Ok(result)
    }
}
