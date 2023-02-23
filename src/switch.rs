use crate::config::LazyConfig;
use crate::input::{get_or_prompt_for_service, get_or_prompt_for_target_identity};
use crate::{cargo, git};
use anyhow::anyhow;
use cargo::CARGO_SERVICE;
use clap::{arg, ArgAction, ArgMatches, Command};
use git::GIT_SERVICE;

pub fn configure_command() -> Command {
    Command::new("switch")
        .about("Switch identity")
        .arg(
            arg!(-s --service "The service to switch profile for")
                .action(ArgAction::Set)
                .value_name("SERVICE")
                .num_args(1),
        )
        .arg(
            arg!(-i --identity "The ID of the identity to switch to")
                .action(ArgAction::Set)
                .value_name("ID")
                .num_args(1),
        )
}

pub fn run_switch(config: &mut LazyConfig, arg_matches: &ArgMatches) -> anyhow::Result<()> {
    config.required()?;

    let service = get_or_prompt_for_service(arg_matches)?;

    match service.as_str() {
        GIT_SERVICE => {
            return Err(anyhow!("Profile switching is not supported for Git, use Git commands to change your settings"));
        }
        CARGO_SERVICE => {
            let identity_config = get_or_prompt_for_target_identity(config, arg_matches)?;
            let identity = identity_config.identity_for_service(CARGO_SERVICE)?;

            if let Some(id) = identity {
                cargo::run_switch(config, id)?
            } else {
                return Err(anyhow!(
                    "Selected identity does not have a {} account",
                    service
                ));
            }
        }
        service => return Err(anyhow!("Unknown service {}", service)),
    }

    Ok(())
}
