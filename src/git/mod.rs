mod check;
mod cli;
mod common;
mod credentials;
mod hook;
mod install;
mod run;
mod whoami;

pub use cli::configure;
pub use run::run_git;
pub use whoami::run_who_am_i;

pub const GIT_SERVICE: &str = "git";
