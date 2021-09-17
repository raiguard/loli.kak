use itertools::Itertools;
use once_cell::sync::OnceCell;
use regex::{Match, Regex};
use std::collections::HashSet;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use std::string;
use thiserror::Error;

use crate::util;

pub type Lists = HashSet<LocationList>;

#[derive(Debug)]
pub struct LocationList {
    pub locations: Vec<Location>,
    pub name: String,
}

impl LocationList {
    pub fn new(name: String, input: String) -> Result<Self, LocationListErr> {
        static LIST_REGEX: OnceCell<Regex> = OnceCell::new();
        let regex = LIST_REGEX.get_or_init(|| {
            Regex::new(r"'(?P<filename>.*?)\|(?P<start_line>\d+)\.(?P<start_column>\d+),(?P<end_line>\d+)\.(?P<end_column>\d+)\|(?P<preview>.*?)'").unwrap()
        });

        let mut locations = Vec::new();

        for captures in regex.captures_iter(&input) {
            match (
                captures.name("filename"),
                [
                    (captures.name("start_line"), captures.name("start_column")),
                    (captures.name("end_line"), captures.name("end_column")),
                ],
                captures.name("preview"),
            ) {
                (
                    Some(filename),
                    [(Some(start_line), Some(start_column)), (Some(end_line), Some(end_column))],
                    Some(preview),
                ) => {
                    let range =
                        KakouneRange::from_matches(start_line, start_column, end_line, end_column)?;
                    locations.push(Location {
                        filename: filename.as_str().to_string(),
                        range,
                        preview: preview.as_str().to_string(),
                    })
                }
                _ => return Err(LocationListErr::InvalidStrList),
            };
        }

        util::kak_print(&format!("{:#?}", locations));
        Ok(LocationList { name, locations })
    }
}

#[derive(Debug, Error)]
pub enum LocationListErr {
    #[error("Invalid source list")]
    InvalidStrList,
    #[error("Invalid range in source list")]
    InvalidRange,
}

#[derive(Debug)]
pub struct Location {
    filename: String,
    range: KakouneRange,
    preview: String,
}

#[derive(Debug)]
pub struct KakounePosition {
    pub line: u32,
    pub column: u32, // in bytes, not chars!!!
}

impl Display for KakounePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.line, self.column)
    }
}

impl FromStr for KakounePosition {
    type Err = LocationListErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line, column) = s.split_once('.').ok_or(LocationListErr::InvalidRange)?;

        Ok(KakounePosition {
            line: line
                .parse::<u32>()
                .map_err(|_| LocationListErr::InvalidRange)?,
            column: column
                .parse::<u32>()
                .map_err(|_| LocationListErr::InvalidRange)?,
        })
    }
}

#[derive(Debug)]
pub struct KakouneRange {
    pub start: KakounePosition,
    pub end: KakounePosition,
}

impl KakouneRange {
    pub fn from_matches(
        start_line: Match,
        start_column: Match,
        end_line: Match,
        end_column: Match,
    ) -> Result<Self, LocationListErr> {
        Ok(KakouneRange {
            start: KakounePosition {
                line: start_line
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| LocationListErr::InvalidRange)?,
                column: start_column
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| LocationListErr::InvalidRange)?,
            },
            end: KakounePosition {
                line: end_line
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| LocationListErr::InvalidRange)?,
                column: end_column
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| LocationListErr::InvalidRange)?,
            },
        })
    }
}

impl FromStr for KakouneRange {
    type Err = LocationListErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(',').ok_or(LocationListErr::InvalidRange)?;

        Ok(KakouneRange {
            start: KakounePosition::from_str(start)?,
            end: KakounePosition::from_str(end)?,
        })
    }
}

impl Display for KakouneRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert!(LocationList::new(
            "foo".to_string(),
            "'src/main.rs|1.5,1.7|lorem ipsum dolor sit amet' 'src/foo|rs|1.5,1.7|LOREM IPSUM DOLOR SIT AMET'".to_string()
        )
        .is_ok());

        let list = LocationList::new(
            "foo".to_string(),
            "'colors/one-darker.kak|11.1,11.4|decl -hidden str fg \"abb2bf\"' 'rc/test.kak|35.13,35.17|face global value \"rgb:%%opt{darkorange}\"'".to_string()
        )
        .unwrap();

        println!("{:#?}", list);
    }
}
