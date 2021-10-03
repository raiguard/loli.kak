use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use crate::location_list::LocationListErr;
use crate::util;

#[derive(Eq, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Highlighter {
    pub filename: String,
    pub scope: HighlighterScope,
}

impl Highlighter {
    pub fn new(filename: &str, scope: HighlighterScope) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            filename: filename.to_string(),
            scope,
        })
    }
    pub fn gen_removal(&self, list_name: &str) -> String {
        format!(
            "edit {}
            remove-highlighter {}/ranges_loli_{}_{}",
            self.filename,
            self.scope,
            list_name,
            util::strip_an(&self.filename)
        )
    }
}

#[derive(Eq, Debug, Deserialize, Hash, Serialize, PartialEq)]
pub enum HighlighterScope {
    Buffer,
    Window,
}

impl Display for HighlighterScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Buffer => "buffer",
                Self::Window => "window",
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub filename: String,
    pub range: KakouneRange,
    pub preview: String,
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
