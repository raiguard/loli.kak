use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[macro_use]

mod context;
mod location_list;
mod types;
mod util;

use context::Context;
use location_list::{Lists, LocationList};

#[derive(StructOpt)]
#[structopt(
    name = "kak-loli",
    about = "An implementation of location lists for kakoune."
)]
pub struct App {
    /// (Optional) The client list to create. If omitted, a global list will be created
    #[structopt(short, long)]
    pub client: Option<String>,

    /// The session that this list is being created for
    #[structopt(short, long)]
    pub session: String,

    /// The filepath of kakoune's current command fifo
    #[structopt(short, long)]
    pub input_fifo: Option<PathBuf>,

    /// The filepath of kakoune's current response fifo
    #[structopt(short, long)]
    pub output_fifo: Option<PathBuf>,

    /// Kakoune's current timestamp for this buffer
    #[structopt(short, long)]
    pub timestamp: Option<usize>,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    /// Prints initialization kakscript
    Init,

    /// Deletes the store for this session
    Clean,

    /// Clears the location list for this client or session
    Clear,

    /// Parses grep-like results into a location list
    Grep { output_path: PathBuf },

    /// Parses a str-list into a location list
    List { input: String },

    /// Highlights the given buffer's location lists
    Highlight { buffer: String },

    /// Jumps to the first item in the location list
    First,

    /// Jumps to the last item in the location list
    Last,
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::from_args();

    match app.cmd {
        Command::Init => init(&app),

        Command::Clean => {
            fs::remove_file(&util::get_store_path(&app.session))?;
        }
        Command::Clear => {
            let ctx = Context::new(&app)?;
            let mut lists = Lists::from_file(&ctx)?;
            lists.clear(&ctx)?;
            lists.write();
        }
        Command::Grep { ref output_path } => {
            let ctx = Context::new(&app)?;
            let list = LocationList::from_grep(&ctx.list_key, fs::read_to_string(output_path)?)?;

            let mut lists = Lists::from_file(&ctx)?;

            lists.insert(list, &ctx)?;
            lists.write();
        }
        Command::List { ref input } => {
            let ctx = Context::new(&app)?;
            let list = LocationList::from_str_list(&ctx.list_key, &input)?;

            let mut lists = Lists::from_file(&ctx)?;

            lists.insert(list, &ctx)?;
            lists.write();
        }
        Command::Highlight { ref buffer } => {
            let ctx = Context::new(&app)?;
            let mut lists = Lists::from_file(&ctx)?;

            lists.highlight(&ctx, &buffer)?;
            lists.write();
        }
        Command::First => {
            let ctx = Context::new(&app)?;
            let mut lists = Lists::from_file(&ctx)?;
            let list = lists.get_mut(&ctx);
            if let Some(list) = list {
                list.navigate(&ctx, 0);
            }
        }
        Command::Last => {
            let ctx = Context::new(&app)?;
            let mut lists = Lists::from_file(&ctx)?;
            let list = lists.get_mut(&ctx);
            if let Some(list) = list {
                list.navigate(&ctx, list.locations.len() - 1);
            }
        }
    }

    Ok(())
}

fn init(app: &App) {
    let local_path = util::get_store_path(&app.session);

    // Creates file and populates it with empty, but valid, data
    Lists::new(&local_path).expect("Could not create session store.");

    // Print kakscript commands for init
    let cmd = env::current_exe().unwrap();
    let cmd = cmd.to_str().unwrap();
    let loli_cmd = format!(
        "set global loli_cmd '{} -s {}'",
        util::editor_escape(cmd),
        app.session
    );

    let script: &str = include_str!("../rc/loli.kak");
    let test: &str = include_str!("../rc/test.kak");

    println!("{}\n{}\n{}", script, test, loli_cmd);
}
