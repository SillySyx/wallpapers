use std::error::Error;

use crate::config::WallpapersConfig;
use crate::background;

pub fn next() -> Result<(), Box<dyn Error>> {
    let config = WallpapersConfig::load()?;
    background::change(config)
}