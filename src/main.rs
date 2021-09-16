#![allow(dead_code, unused)]

use std::env;
use structopt::StructOpt;

mod util;

#[derive(StructOpt)]
#[structopt(
    name = "kak-ll",
    about = "An implementation of location lists for kakoune."
)]
struct App {
    // #[structopt(short, long)]
    // client_name: String,

    // #[structopt(short, long)]
    // session_name: String,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    New,
    Next,
    Prev,
    Init,
}

fn main() {
    let app = App::from_args();

    match app.cmd {
        Command::Init => init(),
        _ => (),
    }
}

fn init() {
    // Inline the contents of the file at compile time
    let script: &str = include_str!("../rc/location-list.kak");
    let cmd = env::current_exe().unwrap();
    let cmd = cmd.to_str().unwrap();
    let ll_cmd = format!("set global ll_cmd '{}'", util::editor_escape(cmd),);
    println!("{}\n{}", script, ll_cmd);
}
