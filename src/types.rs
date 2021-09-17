use once_cell::sync::OnceCell;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use std::string;
use thiserror::Error;

use crate::util;

pub type Lists = HashSet<LocationList>;

pub struct LocationList {
    pub locations: Vec<Location>,
    pub name: String,
}

impl LocationList {
    pub fn new(name: String, input: String) -> Result<Self, LocationListErr> {
        util::kak_print(&input);
        static LIST_REGEX: OnceCell<Regex> = OnceCell::new();
        let regex = LIST_REGEX.get_or_init(|| {
            Regex::new(r"'(?P<filename>.*?)\|(?P<range>.*?)\|(?P<preview>.*?)'").unwrap()
        });

        for captures in regex.captures_iter(&input) {
            match (
                captures.name("filename"),
                captures.name("range"),
                captures.name("preview"),
            ) {
                (Some(filename), Some(range), Some(preview)) => util::kak_print(&format!(
                    "{} / {} / {}",
                    filename.as_str(),
                    range.as_str(),
                    preview.as_str()
                )),
                _ => (),
            }
        }

        Ok(LocationList {
            name,
            locations: Vec::new(),
        })
    }
}

// impl FromStr for LocationList {
//     type Err = std::string::ParseError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {}
// }

#[derive(Debug, Error)]
pub enum LocationListErr {
    #[error("Invalid source list")]
    InvalidStrList,
}

pub struct Location {
    filename: PathBuf,
    range: KakouneRange,
    preview: String,
}

// #[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct KakounePosition {
    pub line: u32,
    pub column: u32, // in bytes, not chars!!!
}

impl Display for KakounePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.line, self.column)
    }
}

// #[derive(Debug, PartialEq)]
pub struct KakouneRange {
    pub start: KakounePosition,
    pub end: KakounePosition,
}

impl Display for KakouneRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{}", self.start, self.end)
    }
}
