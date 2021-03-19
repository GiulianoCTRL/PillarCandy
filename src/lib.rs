//! All enums & structs related to laws

use lazy_static::lazy_static;
use regex::Regex;
use select::document::Document;
use select::predicate::{Name, Predicate};

mod error;
use error::LawErrorKind;

#[allow(dead_code)]
fn law_id_valid(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{4}:[0-9]{1,4}$").unwrap();
    }
    RE.is_match(text)
}

/// Law identifier or "beteckning" of law e.g. 1998:899
#[derive(Debug, Clone, PartialEq)]
struct LawID(String);

#[derive(Debug, Clone, PartialEq)]
pub struct Law {
    year: String,
    number: String,
    title: String,
    sub_title: String,
    doc_type: String,
    sub_type: String,
    department: String,
    date: String,
    published: String,
}

#[allow(dead_code)]
impl LawID {
    /// Analyse if the passed string matches the law regex pattern. If
    /// the pattern matches Result<LawID> will be returned, else Result<E> will
    /// be returned.
    pub fn new(id: &str) -> Result<LawID, LawErrorKind> {
        if law_id_valid(id) {
            Ok(LawID(String::from(id)))
        } else {
            Err(LawErrorKind::IDFormatError)
        }
    }

    /// Create an URL from the law identifier
    fn to_url(&self) -> String {
        format!("http://data.riksdagen.se/dokument/sfs-{}", self.0)
    }
}

#[allow(dead_code)]
impl Law {
    fn fetch_law_data(id: LawID) -> Result<String, LawErrorKind> {
        let r = reqwest::blocking::get(&id.to_url());
        match r {
            Ok(r) => Ok(r.text().unwrap()),
            Err(r) => Err(LawErrorKind::RequestError(r)),
        }
    }

    fn new(id: LawID) -> Result<Law, LawErrorKind> {
        let xml_data = Document::from(Law::fetch_law_data(id)?.as_ref());
        let node = xml_data
            .select(Name("dokumentstatus").descendant(Name("dokument")))
            .next()
            .unwrap();
        let invalid = "NA";
        Ok(Law {
            year: node.select(Name("rm")).next().expect(invalid).text(),
            number: node.select(Name("nummer")).next().expect(invalid).text(),
            title: node.select(Name("titel")).next().expect(invalid).text(),
            sub_title: node.select(Name("subtitel")).next().expect(invalid).text(),
            doc_type: node.select(Name("typ")).next().expect(invalid).text(),
            sub_type: node.select(Name("subtyp")).next().expect(invalid).text(),
            department: node.select(Name("organ")).next().expect(invalid).text(),
            date: node.select(Name("datum")).next().expect(invalid).text(),
            published: node
                .select(Name("publicerad"))
                .next()
                .expect(invalid)
                .text(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirm_valid_law_id() {
        let expected = LawID(String::from("1998:899"));
        let result = LawID::new("1998:899");
        assert_eq!(expected, result.unwrap());
    }
}
