use clap::{ArgMatches, Command};

mod hello;
mod serve;

pub fn configure(command:Command)->Command{
    command
    .subcommand(hello::configure())
    .subcommand(serve::configure())
    .arg_required_else_help(true)
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()>{
    if let Some((cmd,_matches))=matches.subcommand(){
        match cmd{
            hello::COMMAND_NAME=>hello::handle(matches)?,
            serve::COMMAND_NAME=>serve::handle(matches)?,
            &_=>{}
        }
    }
    Ok(())
}