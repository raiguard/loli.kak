use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;
use regex::bytes::Regex;
use std::env;
use std::env::Args;
use std::fmt::Display;
use std::iter::Peekable;
use std::iter::Skip;
use std::str::FromStr;
use thiserror::Error;

const KAKSCRIPT: &str = include_str!("../rc/loli.kak");
const SEPARATOR: &str = "§";

fn main() -> Result<()> {
    let command = parse_args()?;

    match command {
        Command::Init => {
            // Print kakscript commands for init
            let cmd = env::current_exe().unwrap();
            let cmd = cmd.to_str().unwrap();
            println!("{}", KAKSCRIPT);
            println!("set-option global loli_cmd {}", cmd);
        }
        Command::UpdateList {
            bufname,
            list,
            ranges,
        } => {
            let mut ranges = ranges.iter().skip(1);

            let output: String = list
                .iter()
                .filter_map(|location| Location::from_str(location).ok())
                .map(|mut location| {
                    if location.bufname == bufname {
                        let (range, _) = ranges
                            .next()
                            .expect("Range amount mismatch")
                            .split_once("|")
                            .expect("Invalid range format");
                        location.range =
                            KakouneRange::from_str(range).expect("Invalid range format");
                    }
                    location
                })
                .map(|location| format!("{}", location))
                .map(|location| format!("%@{}@", location.replace("@", "@@")))
                .join(" ");

            println!("set-option global loli_global_list {}", output);
        }
    }

    Ok(())
}

fn parse_args() -> Result<Command> {
    let mut args = env::args().skip(1).peekable();
    let subcommand = args.next().expect("Must pass a subcommand.");

    Ok(match subcommand.as_ref() {
        "init" => Command::Init,
        "update-list" => {
            args.next();
            let bufname = args.next().expect("A bufname is required");
            let ranges = parse_list(&mut args)?;
            let list = parse_list(&mut args)?;
            Command::UpdateList {
                bufname,
                ranges,
                list,
            }
        }
        _ => return Err(anyhow!("Unrecognized subcommand: {}", subcommand)),
    })
}

fn parse_list(args: &mut Peekable<Skip<Args>>) -> Result<Vec<String>> {
    let mut list = Vec::new();
    args.next();
    loop {
        let next = args.peek().map(|peeked| peeked.starts_with("--"));
        if let Some(is_key) = next {
            if is_key {
                break;
            } else {
                list.push(args.next().unwrap())
            }
        } else {
            break;
        }
    }

    Ok(list)
}

#[derive(Debug)]
enum Command {
    Init,
    UpdateList {
        bufname: String,
        list: Vec<String>,
        ranges: Vec<String>,
    },
}

#[derive(Debug)]
struct Location {
    bufname: String,
    range: KakouneRange,
    preview: String,
}

impl FromStr for Location {
    type Err = LoliErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: OnceCell
        let regex = Regex::new(r"^(.*?)\|(\d*?.\d*?,\d*?.\d*?)\|(.*)$").unwrap();

        let captures = regex
            .captures(s.as_bytes())
            .ok_or(LoliErr::InvalidLocation)?;

        let bufname = std::str::from_utf8(&captures[1]).map_err(|_| LoliErr::InvalidRange)?;
        let range = std::str::from_utf8(&captures[2]).map_err(|_| LoliErr::InvalidRange)?;
        let preview = std::str::from_utf8(&captures[3]).map_err(|_| LoliErr::InvalidRange)?;

        Ok(Self {
            bufname: bufname.to_string(),
            range: KakouneRange::from_str(range)?,
            preview: preview.to_string(),
        })
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}|{}", self.bufname, self.range, self.preview)
    }
}

#[derive(Debug)]
struct KakouneRange {
    start: KakounePosition,
    end: KakounePosition,
}

impl FromStr for KakouneRange {
    type Err = LoliErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(',').ok_or(LoliErr::InvalidRange)?;

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

#[derive(Debug)]
struct KakounePosition {
    line: usize,
    column: usize,
}

impl Display for KakounePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}", self.line, self.column)
    }
}

impl FromStr for KakounePosition {
    type Err = LoliErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line, column) = s.split_once('.').ok_or(LoliErr::InvalidRange)?;

        Ok(KakounePosition {
            line: line.parse::<usize>().map_err(|_| LoliErr::InvalidRange)?,
            column: column.parse::<usize>().map_err(|_| LoliErr::InvalidRange)?,
        })
    }
}

#[derive(Debug, Error)]
enum LoliErr {
    #[error("Invalid location in list.")]
    InvalidLocation,
    #[error("Invalid range.")]
    InvalidRange,
    #[error("Invalid position.")]
    InvalidPosition,
}
