use std::process::Command;

pub fn get_credentials_helper() -> anyhow::Result<String> {
    Ok(String::from_utf8(
        Command::new("git")
            .args(["config", "--global", "credential.helper"])
            .output()?
            .stdout,
    )?)
}

pub fn get_origin_url() -> anyhow::Result<String> {
    Ok(String::from_utf8(
        Command::new("git")
            .args(["remote", "get-url", "origin"])
            .output()?
            .stdout,
    )?
    .trim_end()
    .to_string())
}
