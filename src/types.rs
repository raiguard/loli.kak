use itertools::Itertools;
use once_cell::sync::OnceCell;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::context::Context;
use crate::util;

#[derive(Debug, Deserialize, Serialize)]
pub struct Lists {
    pub lists: HashMap<String, LocationList>,
    path: PathBuf,
}

impl Lists {
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        let lists = Lists {
            path: path.to_owned(),
            lists: HashMap::new(),
        };
        fs::write(
            path,
            serde_json::to_string(&lists).expect("Could not serialize default store"),
        )
        .expect("Could not write to store");

        Ok(lists)
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = fs::read_to_string(&path).expect("Could not read store file");
        let lists: Lists = serde_json::from_str(&file).expect("Could not deserialize store");
        Ok(lists)
    }

    pub fn insert(&mut self, list: LocationList, ctx: Context) {
        // Clear data for previous entry
        if let Some(existing) = self.lists.get(&list.name) {
            existing.purge_highlighters(ctx);
        }

        self.lists.insert(list.name.clone(), list);
    }

    pub fn write(&self) {
        fs::write(
            &self.path,
            // TODO: Use a more compact format (and don't pretty print when dev is done)
            serde_json::to_string_pretty(&self).expect("Could not serialize store"),
        )
        .expect("Could not write to store");
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocationList {
    pub active_buffers: Vec<String>,
    pub index: u32,
    pub locations: Vec<Location>,
    pub name: String,
}

impl LocationList {
    pub fn from_str_list(name: &str, input: String) -> Result<Self, LocationListErr> {
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

        Ok(LocationList {
            active_buffers: vec![],
            name: name.to_string(),
            locations,
            index: 0,
        })
    }

    pub fn from_grep(name: &str, input: String) -> Result<Self, LocationListErr> {
        static LINE_REGEX: OnceCell<Regex> = OnceCell::new();
        let regex = LINE_REGEX.get_or_init(|| {
            Regex::new(r"^(?P<filename>.*?):(?P<line>\d+):(?P<column>\d+)*:?(?P<preview>.*)$")
                .unwrap()
        });

        let mut locations = Vec::new();

        for line in input.lines() {
            let captures = regex
                .captures(line)
                .ok_or(LocationListErr::InvalidGrepFmt)?;

            let filename = captures
                .name("filename")
                .map(|mtch| mtch.as_str())
                .ok_or(LocationListErr::InvalidGrepFmt)?;

            let line = captures
                .name("line")
                .map(|mtch| {
                    mtch.as_str()
                        .parse::<u32>()
                        .map_err(|_| LocationListErr::InvalidGrepFmt)
                })
                .ok_or(LocationListErr::InvalidGrepFmt)??;

            let column = if let Some(column) = captures.name("column") {
                column
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| LocationListErr::InvalidGrepFmt)?
            } else {
                1
            };

            let preview = captures
                .get(4)
                .map(|mtch| mtch.as_str())
                .ok_or(LocationListErr::InvalidGrepFmt)?
                .trim();

            locations.push(Location {
                filename: filename.to_string(),
                range: KakouneRange {
                    start: KakounePosition { line, column },
                    end: KakounePosition { line, column },
                },
                preview: preview.to_string(),
            })
        }

        Ok(LocationList {
            name: name.to_string(),
            locations,
            index: 0,
            active_buffers: vec![],
        })
    }

    // For now, just generate highlighter ranges so we can see it in action
    // Returns a list of associated files
    pub fn gen_ranges(&self, timestamp: u32) -> Vec<String> {
        let mut options = HashMap::new();

        for (i, location) in self.locations.iter().enumerate() {
            let filename = util::strip_an(&location.filename);

            let mut highlights = options
                .entry(filename.clone() + "_highlight")
                .or_insert_with(Vec::new);
            highlights.push(format!("{}|{}", location.range, "loli_highlight"));

            let mut indices = options
                .entry(filename.clone() + "_indices")
                .or_insert_with(Vec::new);
            indices.push(format!("{}|{}", location.range, i));
        }

        println!(
            "{}",
            options
                .iter()
                .map(|(name, members)| {
                    format!(
                        "decl range-specs loli_{0}_{1}
                        set global loli_{0}_{1} {2} '{3}'",
                        self.name,
                        name,
                        timestamp,
                        members.iter().join("' '")
                    )
                })
                .join("\n")
        );

        options.keys().map(|key| key.to_string()).collect()
    }

    /// Removes all current highlighters for this list
    fn purge_highlighters(&self, ctx: Context) -> Result<(), Box<dyn Error>> {
        ctx.exec_silent(
            self.active_buffers
                .iter()
                .map(|bufname| {
                    format!(
                        "eval -save-regs a %{{
                        execute-keys '\"aZ'
                        edit {0}
                        remove-highlighter buffer/ranges_loli_{1}_{2}_highlight
                        remove-highlighter buffer/ranges_loli_{1}_{2}_indices
                        execute-keys '\"az'
                    }}",
                        bufname,
                        self.name,
                        util::strip_an(bufname)
                    )
                })
                .join("\n"),
        )
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub filename: String,
    range: KakouneRange,
    preview: String,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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
            "foo",
            "'src/main.rs|1.5,1.7|lorem ipsum dolor sit amet' 'src/foo|rs|1.5,1.7|LOREM IPSUM DOLOR SIT AMET'".to_string()
        )
        .is_ok());

        let mut list = Vec::new();
        for _ in 1..1000 {
            list.push("'colors/one-darker.kak|11.1,11.4|decl -hidden str fg \"abb2bf\"'")
        }
        let list_str = list.iter().join(" ");

        let list = LocationList::from_str_list("foo", list_str).unwrap();

        // println!("{:#?}", list);
    }
}
