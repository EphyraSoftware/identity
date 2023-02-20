use std::process::Command;

pub fn get_credentials_helper() -> anyhow::Result<String> {
    Ok(String::from_utf8(
        Command::new("git")
            .args(["config", "--global", "credential.helper"])
            .output()?
            .stdout,
    )?
        .trim_end()
        .to_string())
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

pub fn get_username() -> anyhow::Result<String> {
    Ok(String::from_utf8(
        Command::new("git")
            .args(["config", "user.name"])
            .output()?
            .stdout,
    )?
        .trim_end()
        .to_string())
}

pub fn get_email() -> anyhow::Result<String> {
    Ok(String::from_utf8(
        Command::new("git")
            .args(["config", "user.email"])
            .output()?
            .stdout,
    )?
        .trim_end()
        .to_string())
}
