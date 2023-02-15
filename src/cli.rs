use crate::git;
use clap::{arg, command, ArgAction, Command};

pub fn configure_cli() -> Command {
    command!()
        .arg(arg!(--verify "Verify the content of the config file").action(ArgAction::SetTrue))
        .subcommand(git::cli::configure())
}
