pub mod help;
pub mod run;
pub mod next;

pub fn is(command: &str) -> bool {
    if let Some(arg) = std::env::args().nth(1) {
        if arg == command {
            return true;
        }
    }

    false
}

pub use {
    help::help,
    next::next,
    run::run,
};