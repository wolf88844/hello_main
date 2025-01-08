use clap::{ArgMatches, Command};

pub const COMMAND_NAME: &str = "hello";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Hello world!")
}

pub fn handle(_matches: &ArgMatches) -> anyhow::Result<()> {
    println!("Hello world!");
    Ok(())
}
