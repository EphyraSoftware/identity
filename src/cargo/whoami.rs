use crate::cargo::credentials::get_current_credentials;
use crate::cargo::CARGO_SERVICE;
use crate::config::LazyConfig;
use crate::identity::Identity;
use anyhow::anyhow;

pub fn run_who_am_i(config: &mut LazyConfig) -> anyhow::Result<()> {
    config.required()?;

    let current_credentials = get_current_credentials()?.registry.token;

    let matched_accounts: Vec<Identity> = config
        .identity
        .iter()
        .flat_map(|ic| {
            if let Some(account) = &ic.account {
                let v: Vec<Identity> = account
                    .iter()
                    .filter_map(|ac| {
                        if ac.service == CARGO_SERVICE {
                            Some(Identity::from(ic, ac))
                        } else {
                            None
                        }
                    })
                    .collect();
                v
            } else {
                vec![]
            }
        })
        .filter(|i| i.token() == Some(&current_credentials))
        .collect();

    match matched_accounts.len() {
        0 => {
            return Err(anyhow!(
                "No identity matching the current Cargo credentials"
            ));
        }
        1 => {
            println!("{}", matched_accounts.first().unwrap())
        }
        _ => {
            return Err(anyhow!(
                "Multiple identities matching the current Cargo credentials"
            ));
        }
    }

    Ok(())
}
