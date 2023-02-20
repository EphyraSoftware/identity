use crate::git::common::{get_credentials_helper, get_origin_url};
use crate::identity::Identity;
use anyhow::{anyhow, Context};
use std::io::Write;
use std::process::{ChildStdin, Command, Stdio};
use url::Url;

pub fn get_current_credential(identity: &Identity) -> anyhow::Result<String> {
    if get_credentials_helper()?.is_empty() {
        return Err(anyhow!("No credentials helper configured"));
    }

    let credentials_command = Command::new("git")
        .args(["credential", "fill"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| "Failed to start Git credentials helper")?;
    let mut credentials_command_stdin = credentials_command.stdin.as_ref().unwrap();

    let user = identity.user();
    if user.is_none() {
        return Err(anyhow!("Missing username for identity {}", identity));
    }
    credentials_command_stdin.write_fmt(format_args!("username={}\n", user.unwrap()))?;

    let origin_url = get_origin_url()?;
    if !origin_url.is_empty() {
        credentials_command_stdin.write_fmt(format_args!("url={}\n", origin_url))?;

        write_url_info(&mut credentials_command_stdin, origin_url.as_str())?;
    } else if let Some(mut match_url) = identity.match_url().cloned() {
        if match_url.ends_with('*') {
            match_url.pop();
        }

        write_url_info(&mut credentials_command_stdin, match_url.as_str())?;
    } else {
        return Err(anyhow!(
            "Not in a Git repository and no Match url configured for identity - {}",
            identity
        ));
    };

    // Final newline as end-of-input to the credentials helper
    credentials_command_stdin.write_all(&[b'\n'])?;

    let output = String::from_utf8(credentials_command.wait_with_output()?.stdout)?;
    for line in output.split('\n') {
        let parts: Vec<&str> = line.split('=').collect();
        if Some("password") == parts.first().cloned() {
            return Ok(parts.last().unwrap().to_string());
        }
    }

    Err(anyhow!("No password found"))
}

fn write_url_info(
    credentials_command_stdin: &mut &ChildStdin,
    input_url: &str,
) -> anyhow::Result<()> {
    let input_url_parsed = Url::parse(input_url)?;
    credentials_command_stdin
        .write_fmt(format_args!("protocol={}\n", input_url_parsed.scheme()))?;
    if let Some(h) = input_url_parsed.host_str() {
        credentials_command_stdin.write_fmt(format_args!("host={}\n", h))?;
    } else {
        return Err(anyhow!(
            "Configured url does not have a host - {}",
            input_url
        ));
    }

    Ok(())
}
