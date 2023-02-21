mod credentials;
mod switch;
mod whoami;

pub use switch::run_switch;
pub use whoami::run_who_am_i;

pub const CARGO_SERVICE: &str = "cargo";
