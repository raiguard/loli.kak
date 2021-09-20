use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub struct Context {
    input_fifo: PathBuf,
    output_fifo: PathBuf,
    output_fifo_str: String,
}

impl Context {
    pub fn new(
        input_fifo: Option<PathBuf>,
        output_fifo: Option<PathBuf>,
    ) -> Result<Self, Box<dyn Error>> {
        match (input_fifo, output_fifo) {
            (Some(input_fifo), Some(output_fifo)) => Ok(Self {
                input_fifo,
                output_fifo_str: output_fifo
                    .to_str()
                    .ok_or("Invalid output FIFO path")?
                    .to_string(),
                output_fifo,
            }),
            _ => Err("Missing one or both FIFOs".into()),
        }
    }

    pub fn exec(&self, commands: String) -> Result<String, Box<dyn Error>> {
        fs::write(
            &self.input_fifo,
            format!("echo -to-file {} \"{}\"", self.output_fifo_str, commands),
        )?;
        let output = fs::read_to_string(&self.output_fifo)?;
        Ok(output)
    }

    pub fn exec_silent(&self, commands: String) -> Result<(), Box<dyn Error>> {
        fs::write(
            &self.input_fifo,
            format!("echo -to-file {} \"{}\"", self.output_fifo_str, commands),
        )?;

        Ok(())
    }
}
