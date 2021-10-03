use directories::BaseDirs;
use itertools::Itertools;
use log::LevelFilter;
use simplelog::WriteLogger;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
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

    /// Updates the buffer's range-specs based on changes made
    Update { buffer: String },

    /// Jumps to the first item in the location list
    First,

    /// Jumps to the last item in the location list
    Last,

    /// Jumps to the next item in the location list
    Next,

    /// Jumps to the previous item in the location list
    Prev,

    /// Opens the location list in a special buffer
    Open,
}

fn main() -> Result<(), Box<dyn Error>> {
    create_logger()?;

    // Parse arguments
    let app = App::from_args();

    // Logic
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
        Command::Update { ref buffer } => {
            let ctx = Context::new(&app)?;
            let mut lists = Lists::from_file(&ctx)?;

            // Get all lists that have contents in this buffer
            for (_, list) in lists.lists.iter().filter(|(_, list)| {
                list.highlighters
                    .iter()
                    .any(|highlighter| *buffer == highlighter.filename)
            }) {
                let ranges = ctx.get_option(&format!(
                    "loli_{}_{}_indices",
                    list.name,
                    util::strip_an(&buffer)
                ))?;
                if let Some(ranges) = ranges {
                    log::debug!("{}", ranges);
                }
            }
        }
        Command::First => {
            let ctx = Context::new(&app)?;
            let mut lists = Lists::from_file(&ctx)?;
            let list = lists.get_mut(&ctx);
            if let Some(list) = list {
                list.navigate(&ctx, 0);
            }
            lists.write();
        }
        Command::Last => {
            let ctx = Context::new(&app)?;
            let mut lists = Lists::from_file(&ctx)?;
            let list = lists.get_mut(&ctx);
            if let Some(list) = list {
                list.navigate(&ctx, list.locations.len() - 1);
            }
            lists.write();
        }
        Command::Next => {
            let ctx = Context::new(&app)?;
            let mut lists = Lists::from_file(&ctx)?;
            let list = lists.get_mut(&ctx);
            if let Some(list) = list {
                list.navigate(&ctx, list.index + 1);
            }
            lists.write();
        }
        Command::Prev => {
            let ctx = Context::new(&app)?;
            let mut lists = Lists::from_file(&ctx)?;
            let list = lists.get_mut(&ctx);
            if let Some(list) = list {
                list.navigate(&ctx, list.index - 1);
            }
            lists.write();
        }
        #[allow(unstable_name_collisions)]
        Command::Open => {
            let ctx = Context::new(&app)?;
            let lists = Lists::from_file(&ctx)?;
            if let Some(list) = lists.get(&ctx) {
                let output: String = list
                    .locations
                    .iter()
                    .enumerate()
                    .map(|(index, location)| {
                        format!(
                            "{} {}:{}:{} | {}",
                            if list.index == index { ">" } else { " " },
                            location.filename,
                            location.range.start.line,
                            location.range.start.column,
                            location.preview
                        )
                    })
                    .intersperse("\n".to_string())
                    .collect();

                ctx.cmd(format!(
                    "evaluate-commands -save-regs '\"' -try-client %opt{{toolsclient}} %@
                        edit! -scratch '*{}_locations*'
                        set-register '\"' '{}'
                        execute-keys P{}g
                        set-option buffer filetype loli
                        set-option buffer readonly true
                        try %{{ focus %opt{{toolsclient}} }}
                    @",
                    if list.name == util::DEFAULT_NAME {
                        "global"
                    } else {
                        &list.name
                    },
                    util::editor_escape(&output),
                    list.index + 1
                ));
                log::debug!("{}", output);
            }
        }
    }

    Ok(())
}

fn create_logger() -> Result<(), Box<dyn Error>> {
    // Init logger
    let mut path = BaseDirs::new().unwrap().home_dir().to_path_buf();
    path.push("kak-hang-test.log");

    // Create file if it doesn't already exist
    if !path.exists() {
        File::create(&path)?;
    }

    WriteLogger::init(
        LevelFilter::Trace,
        simplelog::Config::default(),
        OpenOptions::new().append(true).open(&path)?,
    )?;

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
