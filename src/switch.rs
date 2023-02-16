use crate::config::{load_config, update_config, IdentityConfig};
use clap::{ArgMatches, Command};
use inquire::Select;
use std::fmt::{Display, Formatter};

pub fn configure_command() -> Command {
    Command::new("switch")
}

pub fn run_switch(_: &ArgMatches) -> anyhow::Result<()> {
    let cfg = load_config()?;

    let selector = Select::new("Select identity", cfg.identity);
    let identity = selector.prompt()?;

    update_config(move |mut cfg| {
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
