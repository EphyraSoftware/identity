use crate::git::common::{get_email, get_username};

pub fn run_who_am_i() -> anyhow::Result<()> {
    println!("user.name  = {}", get_username()?);
    println!("user.email = {}", get_email()?);
    Ok(())
}
