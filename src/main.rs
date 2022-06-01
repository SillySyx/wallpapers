mod home_folder;
mod commands;
mod color_scheme;
mod background;
mod config;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    if commands::is("help") {
        return commands::help();
    }

    if commands::is("next") {
        return commands::next();
    }

    if commands::is("run") {
        return commands::run();
    }

    commands::help()
}