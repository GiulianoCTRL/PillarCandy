//! All logic for searching laws.
//! All information in this crate was made possible by the dokument-ID API
//! that "sveriges riksdag"'s website provides. More information can be found here:
//! https://data.riksdagen.se/dokumentation/sa-funkar-dokument-id/

use reqwest;
use std::{fmt, num::ParseIntError};

#[allow(dead_code)]
const URL: &str = "http://data.riksdagen.se";
#[allow(dead_code)]
const DOC_QUERY: &str = "dokument";

#[derive(Debug, Clone)]
pub struct ID {
    year: u32,
    num: u32,
}

#[derive(Debug, Clone)]
pub struct Law {
    pub id: ID,
    pub text: String,
}

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

impl ID {
    pub fn new(year: u32, num: u32) -> ID {
        ID { year, num }
    }

    pub fn from_string(s: &str) -> Result<ID, ParseLawError> {
        let mut data = [0u32; 2];

        if s.matches(":").count() == 1 {
            for (index, part) in s.split(":").enumerate() {
                data[index] = part.parse::<u32>()?;
            }
            Ok(ID {
                year: data[0],
                num: data[1],
            })
        } else {
            Err(ParseLawError::InvalidFormat)
        }
    }

    pub fn to_string(&self) -> String {
        [self.year.to_string(), self.num.to_string()].join(":")
    }

    pub fn to_url(&self) -> String {
        let mut url = [URL, DOC_QUERY, "sfs-"].join("/");
        url.push_str(&self.to_string().replace(":", "-"));

        url
    }
}

pub fn get_law(law_id: ID) -> Result<Law, reqwest::Error> {
    let r = reqwest::blocking::get(&law_id.to_url())?;
    if let Err(e) = r.error_for_status_ref() {
        Err(e)
    } else {
        Ok(Law {
            id: law_id,
            text: r.text()?,
        })
    }
}
