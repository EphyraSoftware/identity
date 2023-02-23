use clap::{arg, ArgAction, Command};

pub fn configure() -> Command {
    Command::new("git")
        .about("Manage your Git identities")
        .arg(arg!(-c --check "Check configuration").action(ArgAction::SetTrue))
        .subcommand(
            Command::new("install")
                .arg(arg!(--force "Force the operation to proceed").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("hook")
                .arg(
                    arg!(--"pre-commit" "Behave as a Git pre-commit hook")
                        .action(ArgAction::SetTrue),
                )
                .arg_required_else_help(true),
        )
}
