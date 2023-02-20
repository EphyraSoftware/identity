use crate::config::LazyConfig;
use crate::git::common::get_origin_url;
use crate::git::GIT_SERVICE;
use std::process::{exit, Command};

pub fn run_git_pre_commit_hook(config: &mut LazyConfig) -> anyhow::Result<()> {
    config.required()?;

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

    let identity = config.account_for_url(GIT_SERVICE, origin.as_str())?;
    if identity.user() != username {
        eprintln!(
            "Username mismatch - expected={} != actual={}",
            identity.user(),
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
