use crate::{git, switch, whoami};
use clap::{arg, command, ArgAction, Command};

pub fn configure_cli() -> Command {
    command!()
        .arg(arg!(--verify "Verify the content of the config file").action(ArgAction::SetTrue))
        .subcommand(switch::configure_command())
        .subcommand(whoami::configure_command())
        .subcommand(git::configure())
}
