mod commands;
mod background;
mod config;
mod user;

use std::error::Error;

// Tray icon
// https://github.com/iovxw/ksni
// https://github.com/olback/tray-item-rs

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