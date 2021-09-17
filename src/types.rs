use std::collections::HashSet;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
pub type Lists = HashSet<LocationList>;

pub struct LocationList {
    pub locations: Location,
    pub name: String,
}

impl FromStr for LocationList {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if let Some((range, filename, preview)) = s.matches('|').collect_tuple() {
        //     Self {}
        // }
        todo!()
    }
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
