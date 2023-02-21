use crate::config::LazyConfig;
use crate::git::common::{get_email, get_origin_url, get_username};
use crate::git::GIT_SERVICE;
use std::process::exit;

pub fn run_git_pre_commit_hook(config: &mut LazyConfig) -> anyhow::Result<()> {
    config.required()?;

    let username = get_username()?;

    let email = get_email()?;

    let origin = get_origin_url()?;

    let identity = config.account_for_url(GIT_SERVICE, origin.as_str())?;
    if identity.user() != Some(&username) {
        eprintln!(
            "Username mismatch - expected={} != actual={}",
            identity.user().unwrap_or(&"no username".to_string()),
            username
        );
        exit(1);
    }

    if identity.email() != Some(&email) {
        eprintln!(
            "Email mismatch - expected={} != actual={}",
            identity.email().unwrap_or(&"missing".to_string()),
            email
        );
        exit(1);
    }

    Ok(())
}
