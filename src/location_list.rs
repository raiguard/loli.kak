use itertools::Itertools;
use once_cell::sync::OnceCell;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::context::Context;
use crate::types::*;
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

    pub fn from_file(ctx: &Context) -> Result<Self, Box<dyn Error>> {
        let file = fs::read_to_string(&ctx.store).expect("Could not read store file");
        let lists: Lists = serde_json::from_str(&file).expect("Could not deserialize store");
        Ok(lists)
    }

    pub fn get(&self, ctx: &Context) -> Option<&LocationList> {
        let key = ctx
            .client
            .as_ref()
            .map_or(util::DEFAULT_NAME, |client| &client);
        self.lists.get(key)
    }

    pub fn get_mut(&mut self, ctx: &Context) -> Option<&mut LocationList> {
        let key = ctx
            .client
            .as_ref()
            .map_or(util::DEFAULT_NAME, |client| &client);
        self.lists.get_mut(key)
    }

    pub fn insert(&mut self, mut list: LocationList, ctx: &Context) -> Result<(), Box<dyn Error>> {
        // Clear data for previous entry, if there is one
        self.clear(&ctx)?;

        list.gen_ranges(&ctx)?;

        if list.name == util::DEFAULT_NAME {
            list.highlight_all(&ctx)?;
        }

        self.lists.insert(list.name.clone(), list);

        Ok(())
    }

    pub fn clear(&mut self, ctx: &Context) -> Result<(), Box<dyn Error>> {
        if let Some(existing) = self.lists.get_mut(&ctx.list_key) {
            existing.purge_highlighters();
            self.lists.remove(&ctx.list_key);
        }

        Ok(())
    }

    pub fn highlight(&mut self, ctx: &Context, buffer: &str) -> Result<(), Box<dyn Error>> {
        // Highlight the global list on the buffer level
        if let Some(list) = self.lists.get_mut(util::DEFAULT_NAME) {
            if list
                .locations
                .iter()
                .any(|location| location.filename == buffer)
            {
                ctx.add_highlighters(&list.name, &buffer, true)?;
                list.highlighters
                    .insert(Highlighter::new(&buffer, HighlighterScope::Buffer)?);
            };
        };

        // Highlight the client list on the window level
        // PANIC: We can unwrap here because this command is always called with a client
        if let Some(list) = self.lists.get_mut(ctx.client.as_ref().unwrap()) {
            if list
                .locations
                .iter()
                .any(|location| location.filename == buffer)
            {
                ctx.add_highlighters(&list.name, &buffer, false)?;
                list.highlighters
                    .insert(Highlighter::new(&buffer, HighlighterScope::Window)?);
            };
        };

        Ok(())
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
    pub highlighters: HashSet<Highlighter>,
    pub index: usize,
    pub locations: Vec<Location>,
    pub name: String,
}

impl LocationList {
    pub fn from_str_list(name: &str, input: &str) -> Result<Self, LocationListErr> {
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
            index: 0,
            locations,
            name: name.to_string(),
            highlighters: HashSet::new(),
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
            highlighters: HashSet::new(),
            index: 0,
            locations,
            name: name.to_string(),
        })
    }

    /// Generates the range-specs used for highlighting and tracking
    pub fn gen_ranges(&mut self, ctx: &Context) -> Result<(), Box<dyn Error>> {
        let mut options = HashMap::new();

        for (i, location) in self.locations.iter().enumerate() {
            let filename = util::strip_an(&location.filename);

            let highlights = options
                .entry(filename.clone() + "_highlight")
                .or_insert_with(Vec::new);
            highlights.push(format!("{}|{}", location.range, "loli_highlight"));

            let indices = options
                .entry(filename.clone() + "_indices")
                .or_insert_with(Vec::new);
            indices.push(format!("{}|{}", location.range, i));
        }

        ctx.exec(
            options
                .iter()
                .map(|(name, members)| {
                    format!(
                        "decl range-specs loli_{0}_{1}
                        set global loli_{0}_{1} {2} '{3}'",
                        self.name,
                        name,
                        // PANIC: We can unwrap this, because we guarantee that timestamp will exist when this function is called
                        ctx.timestamp.unwrap(),
                        members.iter().join("' '")
                    )
                })
                .join("\n"),
        )?;

        Ok(())
    }

    /// Highlights the global list in all buffers
    fn highlight_all(&mut self, ctx: &Context) -> Result<(), Box<dyn Error>> {
        // Only do this if this is the global list
        if self.name != util::DEFAULT_NAME {
            return Ok(());
        }

        let buffers = ctx.get_str_list("buflist")?;
        let files: Vec<String> = self
            .locations
            .iter()
            .map(|location| util::strip_an(&location.filename))
            .dedup()
            .collect();

        for (filename, stripped) in buffers
            .iter()
            .map(|bufname| (bufname, util::strip_an(&bufname)))
            .filter(|(_, stripped)| files.contains(stripped))
        {
            println!(
                "eval -save-regs a -draft %{{
                    exec '\"aZ'
                    edit {0}
                    add-highlighter -override buffer/ ranges loli_{1}_{2}_highlight
                    add-highlighter -override buffer/ ranges loli_{1}_{2}_indices
                    exec '\"az'
                }}",
                filename, self.name, stripped
            );

            self.highlighters
                .insert(Highlighter::new(&filename, HighlighterScope::Buffer)?);
        }

        Ok(())
    }

    /// Removes all current highlighters for this list
    fn purge_highlighters(&mut self) {
        println!(
            "evaluate-commands -save-regs a -draft %{{
                {}
            }}",
            self.highlighters
                .iter()
                .map(|highlighter| highlighter.gen_removal(&self.name))
                .join("\n")
        );
    }

    pub fn navigate(&mut self, ctx: &Context, index: usize) {
        // TODO: Show an error in kak when trying to go past the end of a list
        let index = index.clamp(0, self.locations.len() - 1);
        self.index = index;

        if let Some(location) = self.locations.get(index) {
            ctx.cmd(format!(
                "edit {} {} {}",
                location.filename, location.range.start.line, location.range.start.column,
            ))
        }
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

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert!(LocationList::from_str_list(
            "foo",
            "'src/main.rs|1.5,1.7|lorem ipsum dolor sit amet' 'src/foo|rs|1.5,1.7|LOREM IPSUM DOLOR SIT AMET'"
        )
        .is_ok());

        let mut list = Vec::new();
        for _ in 1..1000 {
            list.push("'colors/one-darker.kak|11.1,11.4|decl -hidden str fg \"abb2bf\"'")
        }
        let list_str = list.iter().join(" ");

        let list = LocationList::from_str_list("foo", &list_str).unwrap();

        // println!("{:#?}", list);
    }
}
