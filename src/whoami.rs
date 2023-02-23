use crate::cargo::CARGO_SERVICE;
use crate::config::LazyConfig;
use crate::git::GIT_SERVICE;
use crate::input::get_or_prompt_for_service;
use crate::{cargo, git};
use anyhow::anyhow;
use clap::{arg, ArgAction, ArgMatches, Command};

pub fn configure_command() -> Command {
    Command::new("whoami")
        .about("Determine and display the current identity")
        .arg(
            arg!(-s --service "The service to check")
                .action(ArgAction::Set)
                .value_name("SERVICE")
                .num_args(1),
        )
        .arg(
            arg!(-i --identity "The ID of the identity to check")
                .action(ArgAction::Set)
                .value_name("ID")
                .num_args(1),
        )
}

pub fn run_who_am_i(config: &mut LazyConfig, arg_matches: &ArgMatches) -> anyhow::Result<()> {
    config.required()?;

    let service = get_or_prompt_for_service(arg_matches)?;

    match service.as_str() {
        GIT_SERVICE => git::run_who_am_i()?,
        CARGO_SERVICE => cargo::run_who_am_i(config)?,
        service => return Err(anyhow!("Unknown service {}", service)),
    }

    Ok(())
}
