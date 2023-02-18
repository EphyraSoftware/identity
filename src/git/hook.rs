use crate::config::load_config;
use crate::git::common::get_origin_url;
use std::process::{exit, Command};

pub fn run_git_pre_commit_hook() -> anyhow::Result<()> {
    let config = load_config()?;

    let username = String::from_utf8(
        Command::new("git")
            .args(["config", "user.name"])
            .output()?
            .stdout,
    )?
    .trim_end()
    .to_string();

    let email = String::from_utf8(
        Command::new("git")
            .args(["config", "user.email"])
            .output()?
            .stdout,
    )?
    .trim_end()
    .to_string();

    let origin = get_origin_url()?;

    let candidate_identities = config.identities_for_url(origin.as_str());
    if candidate_identities.len() != 1 {
        eprintln!("Multiple candidate identities - {:?}", candidate_identities);
        exit(1);
    }

    let identity = candidate_identities.first().unwrap();
    if identity.user != username {
        eprintln!(
            "Username mismatch - expected={} != actual={}",
            identity.user, username
        );
        exit(1);
    }

    if identity.email.as_ref() != Some(&email) {
        eprintln!(
            "Email mismatch - expected={} != actual={}",
            identity.email.as_ref().unwrap_or(&"missing".to_string()),
            email
        );
        exit(1);
    }

    Ok(())
}
