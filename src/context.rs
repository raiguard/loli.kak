use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::util;

pub struct Context {
    input_fifo: PathBuf,
    output_fifo: PathBuf,
    output_fifo_str: String,
    session: String,
    pub store: PathBuf,
}

impl Context {
    pub fn new(
        input_fifo: Option<PathBuf>,
        output_fifo: Option<PathBuf>,
        session: String,
    ) -> Result<Self, Box<dyn Error>> {
        match (input_fifo, output_fifo) {
            (Some(input_fifo), Some(output_fifo)) => Ok(Self {
                input_fifo,
                output_fifo_str: output_fifo
                    .to_str()
                    .ok_or("Invalid output FIFO path")?
                    .to_string(),
                output_fifo,
                session,
                store: util::get_store_path(&session),
            }),
            _ => Err("Missing one or both FIFOs".into()),
        }
    }

    pub fn exec(&self, commands: String) -> Result<(), Box<dyn Error>> {
        if !commands.is_empty() {
            fs::write(&self.input_fifo, commands)?;
        }

        Ok(())
    }
}

/// Prints to the kakoune debug log, using the same syntax as `println!`.
#[allow(unused)]
macro_rules! kak_print {
    ($literal:expr) => {
        println!("echo -debug '{}'", $literal)
    };
    ($template:expr, $($arg:tt)*) => ({
        println!("echo -debug '{}'", $crate::util::editor_escape(&format!($template, $($arg)*)));
    })
}
