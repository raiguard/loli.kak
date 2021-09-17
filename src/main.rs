#![allow(dead_code, unused)]

use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

mod types;
mod util;

use types::*;

#[derive(StructOpt)]
#[structopt(
    name = "kak-ll",
    about = "An implementation of location lists for kakoune."
)]
struct App {
    #[structopt(short, long)]
    client_name: Option<String>,

    #[structopt(short, long)]
    session_name: String,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Creates a new location list
    New {
        list: String,
    },
    Next,
    Prev,
    /// Prints initialization kakscript
    Init,
}

fn main() {
    let app = App::from_args();

    match app.cmd {
        Command::Init => init(&app),
        Command::New { list } => {
            println!("echo -debug '{}'", util::editor_escape(&list))
            // let option_name = match app.client_name {
            //     Some(client_name) => client_name,
            //     None => "LLGLOBAL".to_string(),
            // };
            // println!("echo -debug '{}'", util::editor_escape(&option_name));
        }
        _ => (),
    }
}

fn init(app: &App) {
    // Inline the contents of the file at compile time
    let script: &str = include_str!("../rc/location-list.kak");
    let cmd = env::current_exe().unwrap();
    let cmd = cmd.to_str().unwrap();
    let ll_cmd = format!(
        "set global ll_cmd '{} -s {}'",
        util::editor_escape(cmd),
        app.session_name
    );

    println!("{}\n{}", script, ll_cmd);
}
