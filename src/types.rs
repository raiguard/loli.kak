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
    pub fn from_str_list(name: String, input: String) -> Result<Self, LocationListErr> {
        static LIST_REGEX: OnceCell<Regex> = OnceCell::new();
        let regex = LIST_REGEX
            .get_or_init(|| Regex::new(r"'(.*?)\|(\d+)\.(\d+),(\d+)\.(\d+)\|(.*?)'").unwrap());

        let mut locations = Vec::new();

        for captures in regex.captures_iter(&input) {
            let captures: Vec<&str> = captures
                .iter()
                .skip(1)
                .map(|capture| capture.unwrap().as_str())
                .collect();

            if let [filename, start_line, start_column, end_line, end_column, preview] =
                captures[..]
            {
                let range =
                    KakouneRange::from_parts(start_line, start_column, end_line, end_column)?;
                locations.push(Location {
                    filename: filename.to_string(),
                    range,
                    preview: preview.to_string(),
                });
            } else {
                return Err(LocationListErr::InvalidStrList);
            }
        }

        Ok(LocationList { name, locations })
    }

    pub fn from_grep(name: String, input: String) -> Result<Self, LocationListErr> {
        static LINE_REGEX: OnceCell<Regex> = OnceCell::new();
        let regex = LINE_REGEX.get_or_init(|| Regex::new(r"^(.*):(\d+):(\d+):(.*)$").unwrap());

        let mut locations = Vec::new();

        for line in input.lines() {
            let captures = regex
                .captures(line)
                .ok_or(LocationListErr::InvalidGrepFmt)?;

            let filename = captures
                .get(1)
                .map(|mtch| mtch.as_str())
                .ok_or(LocationListErr::InvalidGrepFmt)?;

            let line = captures
                .get(2)
                .map(|mtch| {
                    mtch.as_str()
                        .parse::<u32>()
                        .map_err(|_| LocationListErr::InvalidGrepFmt)
                })
                .ok_or(LocationListErr::InvalidGrepFmt)??;

            let column = captures
                .get(3)
                .map(|mtch| {
                    mtch.as_str()
                        .parse::<u32>()
                        .map_err(|_| LocationListErr::InvalidGrepFmt)
                })
                .ok_or(LocationListErr::InvalidGrepFmt)??;

            let preview = captures
                .get(4)
                .map(|mtch| mtch.as_str())
                .ok_or(LocationListErr::InvalidGrepFmt)?
                .trim();

            locations.push(Location {
                filename: filename.to_string(),
                range: KakouneRange {
                    start: KakounePosition { line, column },
                    end: KakounePosition {
                        line,
                        column: column + 1,
                    },
                },
                // range: KakouneRange::from_parts(line, column, line, column + 1),
                preview: preview.to_string(),
            })
        }

        Ok(LocationList { name, locations })
    }
}

#[derive(Debug, Error)]
pub enum LocationListErr {
    #[error("Invalid source list")]
    InvalidStrList,
    #[error("Invalid range in source list")]
    InvalidRange,
    #[error("Invalid grep format")]
    InvalidGrepFmt,
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

// impl FromStr for KakounePosition {
//     type Err = LocationListErr;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let (line, column) = s.split_once('.').ok_or(LocationListErr::InvalidRange)?;

//         Ok(KakounePosition {
//             line: line
//                 .parse::<u32>()
//                 .map_err(|_| LocationListErr::InvalidRange)?,
//             column: column
//                 .parse::<u32>()
//                 .map_err(|_| LocationListErr::InvalidRange)?,
//         })
//     }
// }

#[derive(Debug)]
pub struct KakouneRange {
    pub start: KakounePosition,
    pub end: KakounePosition,
}

impl KakouneRange {
    pub fn from_parts(
        start_line: &str,
        start_column: &str,
        end_line: &str,
        end_column: &str,
    ) -> Result<Self, LocationListErr> {
        Ok(KakouneRange {
            start: KakounePosition {
                line: start_line
                    .parse::<u32>()
                    .map_err(|_| LocationListErr::InvalidRange)?,
                column: start_column
                    .parse::<u32>()
                    .map_err(|_| LocationListErr::InvalidRange)?,
            },
            end: KakounePosition {
                line: end_line
                    .parse::<u32>()
                    .map_err(|_| LocationListErr::InvalidRange)?,
                column: end_column
                    .parse::<u32>()
                    .map_err(|_| LocationListErr::InvalidRange)?,
            },
        })
    }
}

// impl FromStr for KakouneRange {
//     type Err = LocationListErr;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let (start, end) = s.split_once(',').ok_or(LocationListErr::InvalidRange)?;

//         Ok(KakouneRange {
//             start: KakounePosition::from_str(start)?,
//             end: KakounePosition::from_str(end)?,
//         })
//     }
// }

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
        assert!(LocationList::from_str_list(
            "foo".to_string(),
            "'src/main.rs|1.5,1.7|lorem ipsum dolor sit amet' 'src/foo|rs|1.5,1.7|LOREM IPSUM DOLOR SIT AMET'".to_string()
        )
        .is_ok());

        let mut list = Vec::new();
        for _ in 1..1000 {
            list.push("'colors/one-darker.kak|11.1,11.4|decl -hidden str fg \"abb2bf\"'")
        }
        let list_str = list.iter().join(" ");

        let list = LocationList::from_str_list("foo".to_string(), list_str).unwrap();

        // println!("{:#?}", list);
    }
}
