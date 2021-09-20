#![allow(dead_code, unused)]

use directories::BaseDirs;
use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[macro_use]
mod util;

mod types;

use types::*;

#[derive(StructOpt)]
#[structopt(
    name = "kak-loli",
    about = "An implementation of location lists for kakoune."
)]
struct App {
    /// (Optional) The client list to create. If omitted, a global list will be created
    #[structopt(short, long)]
    client_name: Option<String>,

    /// The session that this list is being created for
    #[structopt(short, long)]
    session_name: String,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Prints initialization kakscript
    Init,
    /// Cleans up (deletes) the store file for the given session
    Clean,
    /// Prints highlighters for the current buffer, if any
    Highlight {
        /// The buffer name
        bufname: String,
        /// The cilent name
        // TODO: This is redundant!
        client: String,
    },
    /// Creates a new location list based on grep output
    Grep {
        /// The current kakoune timestamp
        #[structopt(short, long)]
        timestamp: u32,
        /// The file containing the grep output. Grep output must include column numbers
        filename: PathBuf,
        /// All of the currently open buffers
        #[structopt(short, long)]
        buffers: Vec<String>,
        /// The current buffer name
        #[structopt(long)]
        this_buffer: String,
    },
    /// Creates a new location list from a str-list kakoune setting
    New {
        /// The current kakoune timestamp
        #[structopt(short, long)]
        timestamp: u32,
        /// The contents of the `str-list` option
        list: String,
    },
}

const DEFAULT_NAME: &str = "LOLIGLOBAL";

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::from_args();

    let list_key = match app.client_name {
        Some(ref client_name) => client_name.clone(),
        None => DEFAULT_NAME.to_string(),
    };

    match app.cmd {
        Command::Init => init(&app),
        Command::Clean => {
            fs::remove_file(get_local_path(&app.session_name))
                .expect("Could not delete store file");
        }
        Command::Highlight { bufname, client } => {
            let mut lists = Lists::from_file(&get_local_path(&app.session_name))?;

            // Highlight global list on the buffer level
            if let Some(loli) = lists.lists.get(DEFAULT_NAME) {
                if loli
                    .locations
                    .iter()
                    .map(|location| &location.filename)
                    .contains(&bufname)
                {
                    println!(
                        "add-highlighter -override buffer/ ranges loli_{}_{}_highlight",
                        list_key,
                        util::strip_an(&bufname)
                    )
                };
            }

            // Highlight client list on the window level
            if let Some(loli) = lists.lists.get(&client) {
                if loli
                    .locations
                    .iter()
                    .map(|location| &location.filename)
                    .contains(&bufname)
                {
                    println!(
                        "add-highlighter -override window/ ranges loli_{}_{}_highlight",
                        client,
                        util::strip_an(&bufname)
                    )
                };
            }
        }
        Command::Grep {
            buffers,
            filename,
            this_buffer,
            timestamp,
        } => {
            let mut lists = Lists::from_file(&get_local_path(&app.session_name))?;
            let input = fs::read_to_string(filename)?;
            let list = LocationList::from_grep(&list_key, input)?;
            let files = list.gen_ranges(timestamp);
            if list_key == DEFAULT_NAME {
                global_highlight_open_buffers(&files, &buffers, &list_key);
            } else {
                println!(
                    "add-highlighter -override window/ ranges loli_{}_{}_highlight",
                    list_key,
                    util::strip_an(&this_buffer)
                )
            }
            lists.lists.insert(list_key, list);
            lists.write();
        }
        _ => (),
    };

    Ok(())
}

fn init(app: &App) {
    let local_path = get_local_path(&app.session_name);

    // Creates file and populates it with empty, but valid, data
    Lists::new(&local_path);

    // Print kakscript commands for init
    let cmd = env::current_exe().unwrap();
    let cmd = cmd.to_str().unwrap();
    let loli_cmd = format!(
        "set global loli_cmd '{} -s {}'",
        util::editor_escape(cmd),
        app.session_name
    );

    let script: &str = include_str!("../rc/loli.kak");
    let lgrep: &str = include_str!("../rc/lgrep.kak");
    let test: &str = include_str!("../rc/test.kak");

    println!("{}\n{}\n{}\n{}", script, lgrep, test, loli_cmd);
}

fn get_local_path(session: &str) -> PathBuf {
    // Create or re-create the store file
    // TODO: Have multiple stores for specific lists to improve performance
    let local_path = BaseDirs::new().expect("Could not load local directory");

    let local_path = local_path
        .data_local_dir()
        .to_str()
        .expect("Could not convert local data path");

    let mut local_path: PathBuf = [local_path, "kak", "loli"].iter().collect();
    if !local_path.exists() {
        fs::create_dir_all(&local_path);
    }
    local_path.push(format!("loli-store-{}", session));

    local_path
}

fn global_highlight_open_buffers(files: &Vec<String>, buffers: &Vec<String>, list_key: &str) {
    for (filename, stripped) in buffers
        .iter()
        .map(|bufname| (bufname, util::strip_an(&bufname)))
        .filter(|(_, stripped)| files.contains(stripped))
    {
        println!(
            "eval -save-regs a %{{
                exec '\"aZ'
                edit {}
                add-highlighter -override buffer/ ranges loli_{}_{}_highlight
                exec '\"az'
            }}",
            filename, list_key, stripped
        )
    }
}
