use crate::config::verify_config;
use crate::git::run::run_git;
use crate::switch::run_switch;

mod git;

mod cli;
mod config;
mod switch;

fn main() -> anyhow::Result<()> {
    let matches = cli::configure_cli().get_matches();

    if matches.get_flag("verify") {
        let cfg = config::load_config()?;
        verify_config(&cfg)?;
        return Ok(());
    }

    match matches.subcommand() {
        Some(("git", sub_matches)) => run_git(sub_matches),
        Some(("switch", sub_matches)) => run_switch(sub_matches),
        _ => {
            println!("Unknown command");
            Ok(())
        }
    }
}
