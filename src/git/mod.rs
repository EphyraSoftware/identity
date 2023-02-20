pub mod cli;
pub mod run;

mod check;
mod common;
mod credentials;
mod hook;
mod install;
mod whoami;

pub use whoami::run_who_am_i;

pub const GIT_SERVICE: &str = "git";
