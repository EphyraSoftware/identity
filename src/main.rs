use crate::git::run::run_git;

mod git;

mod cli;
mod config;

fn main() -> anyhow::Result<()> {
    let matches = cli::configure_cli().get_matches();

    if matches.get_flag("verify") {
        config::load_config()?;
        return Ok(());
    }

    match matches.subcommand() {
        Some(("git", sub_matches)) => run_git(sub_matches),
        _ => {
            println!("Unknown command");
            Ok(())
        }
    }
}
