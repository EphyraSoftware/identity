use crate::git::hook::run_git_pre_commit_hook;
use crate::git::install::get_pre_commit_hook_path;
use anyhow::{anyhow, Context};
use regex::Regex;
use std::fs::File;
use std::io::Read;
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

    check_hook_content()?;

    run_git_pre_commit_hook()?;

    println!("Everything looks good!");

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
    if !content.contains("identity git --pre-commit-hook") {
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

fn check_credentials_helper() -> anyhow::Result<String> {
    Ok(String::from_utf8(
        Command::new("git")
            .args(["config", "--global", "credential.helper"])
            .output()?
            .stdout,
    )?)
}
