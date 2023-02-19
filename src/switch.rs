use crate::config::LazyConfig;
use clap::{ArgMatches, Command};
use inquire::Select;

pub fn configure_command() -> Command {
    Command::new("switch")
}

pub fn run_switch(config: &mut LazyConfig, _: &ArgMatches) -> anyhow::Result<()> {
    config.required()?;

    let selector = Select::new("Select identity", config.identity.clone());
    let identity = selector.prompt()?;

    config.update(move |mut cfg| {
        cfg.current_identity = Some(identity.id.clone());
        Ok(cfg)
    })?;

    Ok(())
}
