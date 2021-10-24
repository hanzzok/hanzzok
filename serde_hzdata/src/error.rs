use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<'a> {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error")]
    Nom(nom::Err<nom::error::Error<&'a [u8]>>),
    #[error("{0}")]
    Custom(String),
}

impl<'a> From<nom::Err<nom::error::Error<&'a [u8]>>> for Error<'a> {
    fn from(error: nom::Err<nom::error::Error<&'a [u8]>>) -> Self {
        Error::Nom(error)
    }
}

impl serde::ser::Error for Error<'_> {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Custom(msg.to_string())
    }
}

impl serde::de::Error for Error<'_> {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Custom(msg.to_string())
    }
}
