use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::util;
use crate::App;

/// Prints to the kakoune debug log, using the same syntax as `println!`.
#[allow(unused)]
macro_rules! kak_print {
    ($literal:expr) => {
        println!("echo -debug '{}'", $crate::util::editor_escape(&$literal))
    };
    ($template:expr, $($arg:tt)*) => ({
        println!("echo -debug '{}'", $crate::util::editor_escape(&format!($template, $($arg)*)));
    })
}

pub struct Context {
    pub client: Option<String>,
    input_fifo: PathBuf,
    pub output_fifo: PathBuf,
    pub output_fifo_str: String,
    // session: String,
    pub store: PathBuf,
    pub list_key: String,
    pub timestamp: Option<usize>,
}

impl Context {
    pub fn new(app: &App) -> Result<Self, Box<dyn Error>> {
        match (&app.input_fifo, &app.output_fifo) {
            (Some(input_fifo), Some(output_fifo)) => Ok(Self {
                input_fifo: input_fifo.clone(),
                output_fifo_str: output_fifo
                    .to_str()
                    .ok_or("Invalid output FIFO path")?
                    .to_string(),
                output_fifo: output_fifo.clone(),
                store: util::get_store_path(&app.session),
                list_key: match app.client {
                    Some(ref client) => client.to_string(),
                    None => util::DEFAULT_NAME.to_string(),
                },
                client: app.client.clone(),
                timestamp: app.timestamp,
                // session: app.session.clone(),
            }),
            _ => Err("Missing one or both FIFOs".into()),
        }
    }

    /// Executes the given commands immediately, and returns the output
    pub fn exec(&self, mut commands: &str) -> Result<Option<String>, Box<dyn Error>> {
        // // Always write _something_ to the response fifo to ensure that it closes
        // commands.push_str(&format!("\necho -to-file {} ''", self.output_fifo_str));
        fs::write(&self.input_fifo, commands)?;

        // Wait for kak to be done by reading the response fifo
        let response = fs::read_to_string(&self.output_fifo)?;

        if response.is_empty() {
            Ok(None)
        } else {
            Ok(Some(response))
        }
    }

    /// Executes the given commands when the program is terminated
    pub fn cmd(&self, commands: String) {
        println!("{}", commands);
    }

    pub fn add_highlighters(
        &self,
        ctx: &Context,
        key: &str,
        buffer: &str,
        is_global: bool,
    ) -> Result<(), Box<dyn Error>> {
        ctx.cmd(format!(
            "add-highlighter -override {}/ ranges loli_{}_{}",
            if is_global { "buffer" } else { "window" },
            key,
            util::strip_an(&buffer)
        ));

        Ok(())
    }

    pub fn get_str_list(&self, name: &str) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self
            .get_value(&name)?
            .map_or_else(
                || "".to_string(),
                |str| {
                    // Remove the first and last characters
                    let mut chars = str.chars();
                    chars.next();
                    chars.next_back();
                    chars.as_str().to_string()
                },
            )
            .split("\' \'")
            .map(|str| str.to_string())
            .collect())
    }

    pub fn get_value(&self, name: &str) -> Result<Option<String>, Box<dyn Error>> {
        self.exec(&format!(
            "echo -to-file {} %sh{{
                    echo $kak_quoted_{}
                }}",
            self.output_fifo_str, name
        ))
    }

    pub fn get_option(&self, name: &str) -> Result<Option<String>, Box<dyn Error>> {
        self.exec(&format!(
            "echo -to-file {} %sh{{
                    echo $kak_quoted_opt_{}
                }}",
            self.output_fifo_str, name
        ))
    }

    pub fn get_str_list_option(&self, name: &str) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self
            .get_option(&name)?
            .map_or_else(
                || "".to_string(),
                |str| {
                    // Remove the first and last characters
                    let mut chars = str.chars();
                    chars.next();
                    chars.next_back();
                    chars.as_str().to_string()
                },
            )
            .split("\' \'")
            .map(|str| str.to_string())
            .collect())
    }
}
