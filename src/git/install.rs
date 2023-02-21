use anyhow::{anyhow, Context};
use std::fs::{File, Permissions};
use std::io::Write;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

const PRE_COMMIT_HOOK: &str = r#"#!/usr/bin/env bash
identity git hook --pre-commit
"#;

pub fn run_git_install(force: bool) -> anyhow::Result<()> {
    let hook_path = get_pre_commit_hook_path()?;
    if hook_path.exists() && !force {
        return Err(anyhow!(
            "Pre-commit hook already exists, run with --force to overwrite"
        ));
    }

    let mut f = File::create(&hook_path)
        .with_context(|| format!("Failed to create pre-commit hook script - {:?}", hook_path))?;
    #[cfg(target_family = "unix")]
    f.set_permissions(Permissions::from_mode(0o755))?;

    f.write_all(PRE_COMMIT_HOOK.as_bytes())
        .with_context(|| format!("Failed to write pre-commit hook script - {:?}", hook_path))?;

    println!("Hook installed at {:?}", hook_path);

    Ok(())
}

pub fn get_pre_commit_hook_path() -> anyhow::Result<PathBuf> {
    let git_root = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()?
            .stdout,
    )?
    .trim_end()
    .to_string();

    if git_root.is_empty() {
        return Err(anyhow!("Not in a Git repository"));
    }

    let hook_path = PathBuf::from_str(git_root.as_str())?
        .join(".git")
        .join("hooks")
        .join("pre-commit");

    Ok(hook_path)
}
