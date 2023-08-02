mod check;
mod cli;
mod common;
mod credentials;
mod hook;
mod install;
mod run;
mod whoami;
mod switch;

pub use cli::configure;
pub use run::run_git;
pub use whoami::run_who_am_i;
pub use switch::{prepare_switch, apply_switch};

pub const GIT_SERVICE: &str = "git";
