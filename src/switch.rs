use crate::config::{IdentityConfig, LazyConfig};
use clap::{ArgMatches, Command};
use inquire::Select;
use std::fmt::{Display, Formatter};

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

impl Display for IdentityConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, [user={}, email={}, desc={}]",
            self.id,
            self.user,
            self.email.as_ref().unwrap_or(&"no email".to_string()),
            self.description
                .as_ref()
                .unwrap_or(&"no description".to_string())
        )
    }
}
