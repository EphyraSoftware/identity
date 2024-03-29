use crate::config::LazyConfig;
use crate::git::common::{get_credentials_helper, get_origin_url};
use crate::git::credentials::get_current_credential;
use crate::git::hook::run_git_pre_commit_hook;
use crate::git::install::get_pre_commit_hook_path;
use crate::git::GIT_SERVICE;
use anyhow::{anyhow, Context};
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::process::{Command, Stdio};

pub fn run_git_check(config: &mut LazyConfig) -> anyhow::Result<()> {
    let git_version = check_git().with_context(|| "Git not found")?;

    let git_version_okay =
        check_git_version(git_version.as_str()).with_context(|| "Failed to check Git version")?;
    if !git_version_okay {
        return Err(anyhow!("Unsupported Git version - {}", git_version));
    }

    if get_credentials_helper()?.is_empty() {
        return Err(anyhow!("No credentials helper configured"));
    }

    check_hook_content()?;

    check_credentials(config)?;

    run_git_pre_commit_hook(config)?;

    println!("Everything looks good!");

    Ok(())
}

fn check_credentials(config: &mut LazyConfig) -> anyhow::Result<()> {
    config.required()?;

    let origin = get_origin_url()?;
    let identity = config.account_for_url(GIT_SERVICE, origin.as_str())?;

    let token = identity.token();

    if let Some(configured_token) = token {
        let actual_token = get_current_credential(&identity)?;

        if configured_token.as_str() != actual_token.as_str() {
            return Err(anyhow!(
                "The token in your identity.toml does not match the token Git is configured to use"
            ));
        }
    }

    Ok(())
}

fn check_hook_content() -> anyhow::Result<()> {
    let pre_commit_hook_path = get_pre_commit_hook_path()?;
    let mut f = File::open(&pre_commit_hook_path).with_context(|| "Pre-commit hook not found")?;
    let mut content = String::new();
    f.read_to_string(&mut content).with_context(|| {
        format!(
            "Failed to read pre-commit hook - {:?}",
            pre_commit_hook_path
        )
    })?;
    if !content.contains("identity git hook --pre-commit") {
        return Err(anyhow!(
            "Pre-commit hook does not contain an identity check"
        ));
    }

    Ok(())
}

fn check_git() -> anyhow::Result<String> {
    Ok(String::from_utf8(
        Command::new("git").arg("--version").output()?.stdout,
    )?)
}

fn check_git_version(version: &str) -> anyhow::Result<bool> {
    let re = Regex::new(r"(\d+)\.\d+\.\d+")?;
    if let Some(matches) = re.captures(version) {
        let major_version = matches.get(1).map_or("", |m| m.as_str());

        Ok(major_version == "2")
    } else {
        Ok(false)
    }
}

pub fn check_is_git_repository() -> anyhow::Result<()> {
    let code = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?.wait()?;

    if !code.success() {
        return Err(anyhow!("Not in a Git repository"));
    }

    Ok(())
}
