use anyhow::Result;
use pico_args::Arguments;

const KAKSCRIPT: &str = include_str!("../rc/loli.kak");

fn main() -> Result<()> {
    let mut args = Arguments::from_env();
    let subcommand = args.subcommand()?.expect("A subcommand is required");

    println!(
        "{}",
        match subcommand.as_ref() {
            "init" => {
                KAKSCRIPT
            }
            _ => {
                todo!()
            }
        }
    );

    Ok(())
}
