use crate::config::LazyConfig;
use crate::git::check::run_git_check;
use crate::git::hook::run_git_pre_commit_hook;
use crate::git::install::run_git_install;
use anyhow::anyhow;
use clap::ArgMatches;

pub fn run_git(config: &mut LazyConfig, sub_matches: &ArgMatches) -> anyhow::Result<()> {
    if sub_matches.get_flag("check") {
        run_git_check(config)
    } else if sub_matches.get_flag("pre-commit-hook") {
        run_git_pre_commit_hook(config)
    } else if sub_matches.get_flag("install") {
        run_git_install(sub_matches.get_flag("force"))
    } else {
        Err(anyhow!("the command does not do anything on its own"))
    }
}
