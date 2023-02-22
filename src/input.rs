use crate::cargo::CARGO_SERVICE;
use crate::config::{IdentityConfig, LazyConfig};
use crate::git::GIT_SERVICE;
use anyhow::anyhow;
use clap::ArgMatches;
use inquire::Select;

pub fn get_or_prompt_for_service(arg_matches: &ArgMatches) -> anyhow::Result<String> {
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

pub fn get_or_prompt_for_target_identity(
    config: &mut LazyConfig,
    arg_matches: &ArgMatches,
) -> anyhow::Result<IdentityConfig> {
    let target_identity = arg_matches
        .get_one::<String>("identity")
        .map(|id| {
            let identity_configs: Vec<&IdentityConfig> = config.identity.iter().filter(|i| i.id.as_str() == id.as_str()).collect();
            identity_configs.first().cloned().cloned()
        })
        .unwrap_or(None)
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
