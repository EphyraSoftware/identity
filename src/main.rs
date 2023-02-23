#![doc = include_str!("../README.md")]

use crate::config::{verify_config, LazyConfig};
use crate::git::run_git;
use crate::switch::run_switch;
use crate::whoami::run_who_am_i;

mod cargo;
mod git;

mod cli;
mod config;
mod identity;
mod input;
mod switch;
mod whoami;

fn main() -> anyhow::Result<()> {
    let mut config = LazyConfig::new();

    let matches = cli::configure_cli().get_matches();

    if matches.get_flag("verify") {
        verify_config(&mut config)?;
        return Ok(());
    }

    match matches.subcommand() {
        Some(("git", sub_matches)) => run_git(&mut config, sub_matches),
        Some(("switch", sub_matches)) => run_switch(&mut config, sub_matches),
        Some(("whoami", sub_matches)) => run_who_am_i(&mut config, sub_matches),
        _ => {
            cli::configure_cli().print_help()?;
            Ok(())
        }
    }
}
