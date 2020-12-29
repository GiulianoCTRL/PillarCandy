//! All logic for searching laws.
//! All information in this crate was made possible by the dokument-ID API
//! that "sveriges riksdag"'s website provides. More information can be found here:
//! https://data.riksdagen.se/dokumentation/sa-funkar-dokument-id/

use std::fmt;
mod err;

#[allow(dead_code)]
const URL: &str = "http://data.riksdagen.se";
#[allow(dead_code)]
const DOC_QUERY: &str = "dokument";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ID {
    year: u32,
    num: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Law {
    ValidLaw { id: ID, text: String },
    InvalidLaw { id: Option<ID>, error: String },
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.year, self.num)
    }
}

impl ID {
    // fn new(year: u32, num: u32) -> ID {
    //     ID { year, num }
    // }

    fn from_string(s: &str) -> Result<ID, err::ParseLawError> {
        let mut data = [0u32; 2];

        if s.matches(':').count() == 1 {
            for (index, part) in s.split(':').enumerate() {
                data[index] = part.parse::<u32>()?;
            }
            Ok(ID {
                year: data[0],
                num: data[1],
            })
        } else {
            Err(err::ParseLawError::InvalidFormat)
        }
    }

    fn to_url(&self) -> String {
        format!("{}/{}/sfs-{}", URL, DOC_QUERY, &self)
    }
}

impl Law {
    fn get_law(law_id: ID) -> Result<Law, reqwest::Error> {
        let r = reqwest::blocking::get(&law_id.to_url())?;
        if let Err(e) = r.error_for_status_ref() {
            Err(e)
        } else {
            Ok(Law::ValidLaw {
                id: law_id,
                text: r.text()?,
            })
        }
    }

    pub fn from_string(s: &str) -> Law {
        match ID::from_string(s) {
            Ok(i) => match Law::get_law(i) {
                Ok(l) => l,
                Err(e) => Law::InvalidLaw {
                    id: Some(i),
                    error: e.to_string(),
                },
            },
            Err(e) => Law::InvalidLaw {
                id: None,
                error: e.to_string(),
            },
        }
    }

    pub fn from_id(id: ID) -> Law {
        match Law::get_law(id) {
            Ok(l) => l,
            Err(e) => Law::InvalidLaw {
                id: None,
                error: e.to_string(),
            },
        }
    }

    pub fn text(&self) -> String {
        match self {
            Law::ValidLaw { id: _, text: s } => String::from(s),
            Law::InvalidLaw { id: _, error: e } => String::from(e),
        }
    }
}
