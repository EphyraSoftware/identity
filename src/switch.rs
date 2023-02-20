use crate::config::{IdentityConfig, LazyConfig};
use crate::{cargo, git};
use anyhow::anyhow;
use cargo::CARGO_SERVICE;
use clap::{arg, ArgAction, ArgMatches, Command};
use git::GIT_SERVICE;
use inquire::Select;

pub fn configure_command() -> Command {
    Command::new("switch")
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
                cargo::switch_profile(config, id)?
            } else {
                return Err(anyhow!("Selected identity does not have a {} account", service))
            }
        }
        service => return Err(anyhow!("Unknown service {}", service)),
    }

    Ok(())
}

fn get_or_prompt_for_service(arg_matches: &ArgMatches) -> anyhow::Result<String> {
    let service = arg_matches
        .get_one::<String>("service")
        .cloned()
        .or_else(|| prompt_for_service().map_or_else(|_| None, Some));

    if let Some(s) = service {
        Ok(s)
    } else {
        Err(anyhow!("Please specify a service"))
    }
}

fn prompt_for_service() -> anyhow::Result<String> {
    let selector = Select::new("Select service", vec![GIT_SERVICE, CARGO_SERVICE]);
    Ok(selector.prompt()?.to_string())
}

fn get_or_prompt_for_target_identity(config: &mut LazyConfig, arg_matches: &ArgMatches) -> anyhow::Result<IdentityConfig> {
    let target_identity = arg_matches
        .get_one::<IdentityConfig>("identity")
        .cloned()
        .or_else(|| prompt_for_target_identity(config).map_or_else(|_| None, Some));

    if let Some(t) = target_identity {
        Ok(t)
    } else {
        Err(anyhow!("Please specify an identity"))
    }
}

fn prompt_for_target_identity(config: &mut LazyConfig) -> anyhow::Result<IdentityConfig> {
    let selector = Select::new("Select identity", config.identity.clone());
    Ok(selector.prompt()?)
}
