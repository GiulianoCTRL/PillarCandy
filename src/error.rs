//! This module defines all error types for this crate.

use std::{error::Error, fmt};

#[allow(dead_code)]
/// All custom errors associated with Laws
#[derive(Debug)]
pub enum LawErrorKind {
    IDFormatError,
    DataFormatError,
    ParserError,
    RequestError(reqwest::Error),
}

impl Error for LawErrorKind {}

impl fmt::Display for LawErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LawErrorKind::IDFormatError => write!(f, "Valid LawID format is <year>:<number>."),
            LawErrorKind::DataFormatError => write!(f, "Law does not contain valid information."),
            LawErrorKind::ParserError => write!(f, "Placeholder"),
            LawErrorKind::RequestError(err) => err.fmt(f),
        }
    }
}

impl From<reqwest::Error> for LawErrorKind {
    fn from(error: reqwest::Error) -> Self {
        LawErrorKind::RequestError(error)
    }
}
