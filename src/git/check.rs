use crate::git::hook::run_git_pre_commit_hook;
use anyhow::{anyhow, Context};
use regex::Regex;
use std::process::Command;

pub fn run_git_check() -> anyhow::Result<()> {
    let git_version = check_git().with_context(|| "Git not found")?;

    let git_version_okay =
        check_git_version(git_version.as_str()).with_context(|| "Failed to check Git version")?;
    if !git_version_okay {
        return Err(anyhow!("Unsupported Git version - {}", git_version));
    }

    let credentials_helper = check_credentials_helper()?;
    if credentials_helper == "" {
        return Err(anyhow!("No credentials helper configured"));
    }

    run_git_pre_commit_hook()?;

    println!("Everything looks good!");

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

fn check_credentials_helper() -> anyhow::Result<String> {
    Ok(String::from_utf8(
        Command::new("git")
            .args(["config", "--global", "credential.helper"])
            .output()?
            .stdout,
    )?)
}
