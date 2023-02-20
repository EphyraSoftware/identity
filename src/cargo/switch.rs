use crate::cargo::credentials::{get_current_credentials, write_credentials};
use crate::cargo::CARGO_SERVICE;
use crate::config::LazyConfig;
use crate::identity::Identity;
use anyhow::anyhow;
use inquire::Confirm;

pub fn switch_profile(config: &mut LazyConfig, identity: Identity) -> anyhow::Result<()> {
    let configured_token = identity.token();

    if configured_token.is_none() {
        return Err(anyhow!("No Cargo token found for identity {:?}", identity));
    }

    let mut cargo_credentials = get_current_credentials()?;

    if cargo_credentials.registry.token.is_empty() {
        return Err(anyhow!("The token in your Cargo credentials file is empty"));
    }

    if !is_token_known(config, cargo_credentials.registry.token)? {
        let confirm = Confirm::new(
            "The token in your Cargo credentials file is not known, overwrite anyway?",
        );
        if !confirm.prompt()? {
            return Err(anyhow!(
                "Will not overwrite credentials which are not known"
            ));
        }
    }

    cargo_credentials.registry.token = configured_token.unwrap().clone();

    write_credentials(cargo_credentials)?;

    Ok(())
}

fn is_token_known(config: &mut LazyConfig, token: String) -> anyhow::Result<bool> {
    for ic in &config.identity {
        if let Some(t) = ic
            .identity_for_service(CARGO_SERVICE)?
            .map(|i| i.token().cloned())
            .unwrap_or(None)
        {
            if t == token {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
