use std::process::{Command, Stdio};

use anyhow::{Context, anyhow};

use super::check;
use crate::{
    config::LazyConfig,
    git::{common::get_origin_url, GIT_SERVICE},
    identity::Identity,
};

pub fn prepare_switch(config: &mut LazyConfig) -> anyhow::Result<Identity<'_>> {
    check::check_is_git_repository()
        .with_context(|| "Must be in a git repository to switch credentials")?;

    let origin_url = get_origin_url()?;

    config.account_for_url(GIT_SERVICE, origin_url.as_str())
}

pub fn apply_switch(identity: Identity<'_>) -> anyhow::Result<()> {
    let username = identity.user().context("No username found")?;
    let email = identity.email().context("No email found")?;

    set_username(username)?;
    set_email(email)?;

    Ok(())
}

fn set_username(username: &str) -> anyhow::Result<()> {
    let code = Command::new("git")
        .args(["config", "user.name", username])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?.wait()?;

    if !code.success() {
        return Err(anyhow!("Failed to set username"));
    }

    Ok(())
}

fn set_email(email: &str) -> anyhow::Result<()> {
    let code = Command::new("git")
        .args(["config", "user.email", email])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?.wait()?;

    if !code.success() {
        return Err(anyhow!("Failed to set email"));
    }

    Ok(())
}
