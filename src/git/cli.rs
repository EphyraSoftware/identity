use clap::{arg, ArgAction, Command};

pub fn configure() -> Command {
    Command::new("git")
        .about("Manage your Git identities")
        .arg(arg!(-c --check "Check configuration").action(ArgAction::SetTrue))
        .arg(arg!(--install "Install Git hook").action(ArgAction::SetTrue))
        .arg(arg!(--force "Force the operation to proceed").action(ArgAction::SetTrue))
        .arg(arg!(--"pre-commit-hook" "Behave as a Git pre-commit hook").action(ArgAction::SetTrue))
}
