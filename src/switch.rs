use crate::config::LazyConfig;
use crate::input::{get_or_prompt_for_service, get_or_prompt_for_target_identity, prompt_confirm};
use crate::{cargo, git};
use anyhow::{anyhow, Context};
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
            let matched_identity = git::prepare_switch(config).context("Could not find an identity to switch to based on the origin of this git repository")?;

            let confirm = prompt_confirm(format!("Selected identity `{}` based on the git origin, apply? (y/n)", matched_identity.id()).as_str())?;
            if confirm {
                git::apply_switch(matched_identity)?;
                println!("Applied successfully, running `whoami` to verify");

                git::run_who_am_i()?;
            } else {
                println!("Okay, stopping without making changes");
            }
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
