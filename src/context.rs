use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::util;

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

pub struct Context {
    pub client: Option<String>,
    input_fifo: PathBuf,
    output_fifo: PathBuf,
    output_fifo_str: String,
    // session: String,
    pub store: PathBuf,
    pub list_key: String,
    pub timestamp: Option<usize>,
}

impl Context {
    pub fn new(
        input_fifo: Option<PathBuf>,
        output_fifo: Option<PathBuf>,
        session: String,
        client: Option<String>,
        timestamp: Option<usize>,
    ) -> Result<Self, Box<dyn Error>> {
        match (input_fifo, output_fifo) {
            (Some(input_fifo), Some(output_fifo)) => Ok(Self {
                input_fifo,
                output_fifo_str: output_fifo
                    .to_str()
                    .ok_or("Invalid output FIFO path")?
                    .to_string(),
                output_fifo,
                store: util::get_store_path(&session),
                list_key: match client {
                    Some(ref client) => client.to_string(),
                    None => util::DEFAULT_NAME.to_string(),
                },
                client,
                timestamp,
                // session,
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

    pub fn add_highlighters(
        &self,
        key: &str,
        buffer: &str,
        is_global: bool,
    ) -> Result<(), Box<dyn Error>> {
        let command = format!(
            "add-highlighter -override {}/ ranges loli_{}_{}_highlight",
            if is_global { "buffer" } else { "window" },
            key,
            util::strip_an(&buffer)
        );

        kak_print!(&command);

        println!("{}", command);

        Ok(())
        // self.exec(command)
    }
}
