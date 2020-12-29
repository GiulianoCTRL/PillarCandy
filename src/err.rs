use std::{fmt, num::ParseIntError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseLawError {
    InvalidFormat,
    NotAnInt(ParseIntError),
}

#[derive(Debug, Clone, Copy)]
pub struct InvalidFormat;

impl InvalidFormat {
    fn as_str(&self) -> &str {
        "Invalid Format, search format should be <year>:<number>"
    }
}

impl fmt::Display for ParseLawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseLawError::InvalidFormat => {
                write!(f, "Parsing error {:?}", self::InvalidFormat.as_str())
            }
            ParseLawError::NotAnInt(err) => err.fmt(f),
        }
    }
}

impl From<ParseIntError> for ParseLawError {
    fn from(error: ParseIntError) -> Self {
        ParseLawError::NotAnInt(error)
    }
}
