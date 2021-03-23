//! All enums & structs related to laws

use lazy_static::lazy_static;
use regex::Regex;
use select::{document::Document, node::Node};
use select::predicate::{Name, Predicate};

mod error;
use error::LawErrorKind;

/// Helper function to ensure compilation of regex pattern only happens once
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

#[allow(dead_code)]
impl LawID {
    /// Analyse if passed string matches the law regex pattern. If
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

    fn fetch_law_data(&self) -> Result<String, LawErrorKind> {
        let r = reqwest::blocking::get(&self.to_url());
        match r {
            Ok(r) => Ok(r.text().unwrap()),
            Err(r) => Err(LawErrorKind::RequestError(r)),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct LawData{
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


fn get_node_text(node: &Node, name: &str) -> Result<String, LawErrorKind> {
    match node.select(Name(name)).next() {
        Some(n) => Ok(n.text()),
        None => Err(LawErrorKind::LawDataError),
    }
}

#[allow(dead_code)]
impl LawData{
    
    fn new(id: LawID) -> Result<LawData, LawErrorKind> {
        let xml_data = Document::from(id.fetch_law_data()?.as_ref());
        let node = xml_data.select(Name("dokumentstatus").descendant(Name("dokument"))).next().unwrap();
        Ok(LawData{
            year: get_node_text(&node, "rm")?,
            number: get_node_text(&node, "nummer")?,
            title: get_node_text(&node, "titel")?,
            sub_title: get_node_text(&node, "subtitel")?,
            doc_type: get_node_text(&node, "typ")?,
            sub_type: get_node_text(&node, "subtyp")?,
            department: get_node_text(&node, "organ")?,
            date: get_node_text(&node, "datum")?,
            published: get_node_text(&node, "publicerad")?,
        })
    }
}

pub struct Law {
    id: LawID,
    data: LawData,
    refs: Vec<LawID>,
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
