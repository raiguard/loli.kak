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

mod types;
mod util;

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
    /// Creates a new location list based on grep output
    Grep {
        /// The current kakoune timestamp
        #[structopt(short, long)]
        timestamp: u32,
        /// The file containing the grep output. Grep output must include column numbers
        filename: PathBuf,
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

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::from_args();

    let option_name = match app.client_name {
        Some(ref client_name) => client_name.clone(),
        None => "LOLIGLOBAL".to_string(),
    };

    match app.cmd {
        Command::Init => init(&app),
        Command::Clean => {
            fs::remove_file(get_local_path(&app.session_name))
                .expect("Could not delete store file");
        }
        Command::Grep {
            filename,
            timestamp,
        } => {
            let mut lists = Lists::from_file(&get_local_path(&app.session_name))?;
            let input = fs::read_to_string(filename)?;
            let list = LocationList::from_grep(&option_name, input)?;
            list.gen_ranges(timestamp);
            lists.lists.insert(option_name, list);
            lists.write();
        }
        Command::New {
            list: input,
            timestamp,
        } => {
            let mut lists = Lists::from_file(&get_local_path(&app.session_name))?;
            let list = LocationList::from_str_list(&option_name, input)?;
            lists.lists.insert(option_name, list);
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
