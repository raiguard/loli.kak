use directories::BaseDirs;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[macro_use]
mod util;

mod context;
mod types;

use context::Context;
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

    // / The session that this list is being created for
    #[structopt(short, long)]
    session_name: String,

    #[structopt(short, long)]
    input_fifo: Option<PathBuf>,

    #[structopt(short, long)]
    output_fifo: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    /// Prints initialization kakscript
    Init,

    /// Deletes the store for this session
    Clean,

    /// Parse grep-like results into a location list
    Grep { output_path: PathBuf },
}

const DEFAULT_NAME: &str = "LOLIGLOBAL";

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::from_args();

    match app.cmd {
        Command::Init => init(&app),
        Command::Clean => {
            fs::remove_file(&get_local_path(&app.session_name))?;
        }
        Command::Grep { ref output_path } => {
            let ctx = Context::new(app.input_fifo, app.output_fifo)?;

            let list_key = match app.client_name {
                Some(ref client_name) => client_name,
                None => DEFAULT_NAME,
            };
            let list = LocationList::from_grep(list_key, fs::read_to_string(output_path)?)?;

            let mut lists = Lists::from_file(&get_local_path(&app.session_name))?;

            lists.insert(list, ctx);
            lists.write();
        }
    }

    Ok(())
}

fn init(app: &App) {
    let local_path = get_local_path(&app.session_name);

    // Creates file and populates it with empty, but valid, data
    Lists::new(&local_path).expect("Could not create session store.");

    // Print kakscript commands for init
    let cmd = env::current_exe().unwrap();
    let cmd = cmd.to_str().unwrap();
    let loli_cmd = format!(
        "set global loli_cmd '{} -s {}'",
        util::editor_escape(cmd),
        app.session_name
    );

    let script: &str = include_str!("../rc/loli.kak");
    let test: &str = include_str!("../rc/test.kak");

    println!("{}\n{}\n{}", script, test, loli_cmd);
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
        fs::create_dir_all(&local_path).expect("Could not create local data directory");
    }
    local_path.push(format!("loli-store-{}", session));

    local_path
}

// OLD STUFF
//
// fn global_highlight_open_buffers(files: &[String], buffers: &[String], list_key: &str) {
//     for (filename, stripped) in buffers
//         .iter()
//         .map(|bufname| (bufname, util::strip_an(&bufname)))
//         .filter(|(_, stripped)| files.contains(stripped))
//     {
//         println!(
//             "eval -save-regs a %{{
//                 exec '\"aZ'
//                 edit {0}
//                 add-highlighter -override buffer/ ranges loli_{1}_{2}_highlight
//                 add-highlighter -override buffer/ ranges loli_{1}_{2}_indices
//                 exec '\"az'
//             }}",
//             filename, list_key, stripped
//         )
//     }
// }
// /// Cleans up (deletes) the store file for the given session
// Clean,
// /// Prints highlighters for the current buffer, if any
// Highlight {
//     /// The buffer name
//     bufname: String,
//     /// The cilent name
//     // TODO: This is redundant!
//     client: String,
// },
// /// Clears the given location list
// Clear,
// /// Creates a new location list based on grep output
// Grep {
//     /// The current kakoune timestamp
//     #[structopt(short, long)]
//     timestamp: u32,
//     /// The file containing the grep output. Grep output must include column numbers
//     filename: PathBuf,
//     /// All of the currently open buffers
//     #[structopt(short, long)]
//     buffers: Vec<String>,
//     /// The current buffer name
//     #[structopt(long)]
//     this_buffer: String,
// },
// /// Creates a new location list from a str-list kakoune setting
// New {
//     /// The current kakoune timestamp
//     #[structopt(short, long)]
//     timestamp: u32,
//     /// The contents of the `str-list` option
//     list: String,
// },
//
// match app.cmd {
// Command::Init => init(&app),
// Command::Clean => {
//     fs::remove_file(get_local_path(&app.session_name))
//         .expect("Could not delete store file");
// }
// Command::Highlight { bufname, client } => {
//     let mut lists = Lists::from_file(&get_local_path(&app.session_name))?;

//     // Highlight global list on the buffer level
//     if let Some(loli) = lists.lists.get(DEFAULT_NAME) {
//         if loli
//             .locations
//             .iter()
//             .map(|location| &location.filename)
//             .contains(&bufname)
//         {
//             util::add_highlighters(list_key, &bufname, false)
//         };
//     }

//     // Highlight client list on the window level
//     if let Some(loli) = lists.lists.get(&client) {
//         if loli
//             .locations
//             .iter()
//             .map(|location| &location.filename)
//             .contains(&bufname)
//         {
//             util::add_highlighters(&client, &bufname, true)
//         };
//     }
// }
// Command::Clear => {
//     // println!(
//     //     "remove-highlighter {0}/ranges_loli_{1}_{2}_highlight
//     //     remove-highlighter {0}/ranges_loli_{1}_{2}_highlight",
//     //     if list_key == DEFAULT_NAME {
//     //         "buffer"
//     //     } else {
//     //         "window"
//     //     },
//     //     list_key,
//     //     util::strip_an(&bufname)
//     // )
// }
// Command::Grep {
//     buffers,
//     filename,
//     this_buffer,
//     timestamp,
// } => {
//     let mut lists = Lists::from_file(&get_local_path(&app.session_name))?;
//     let input = fs::read_to_string(filename)?;
//     let list = LocationList::from_grep(&list_key, input)?;
//     let files = list.gen_ranges(timestamp);
//     if list_key == DEFAULT_NAME {
//         global_highlight_open_buffers(&files, &buffers, &list_key);
//     } else {
//         util::add_highlighters(list_key, &this_buffer, true)
//     }
//     lists.lists.insert(list_key.to_string(), list);
//     lists.write();
// }
// _ => (),
// };
