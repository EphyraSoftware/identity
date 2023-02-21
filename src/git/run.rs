use crate::config::LazyConfig;
use crate::git::check::run_git_check;
use crate::git::hook::run_git_pre_commit_hook;
use crate::git::install::run_git_install;
use anyhow::anyhow;
use clap::ArgMatches;

pub fn run_git(config: &mut LazyConfig, arg_matches: &ArgMatches) -> anyhow::Result<()> {
    if arg_matches.get_flag("check") {
        return run_git_check(config);
    }

    match arg_matches.subcommand() {
        Some(("install", sub_matches)) => {
            run_git_install(sub_matches.get_flag("force"))
        }
        Some(("hook", sub_matches)) => {
            if sub_matches.get_flag("pre-commit") {
                run_git_pre_commit_hook(config)
            } else {
                Err(anyhow!("Only `--pre-commit` is supported"))
            }
        }
        Some(_) | None => {
            Err(anyhow!("Unknown subcommand"))
        }
    }
}
