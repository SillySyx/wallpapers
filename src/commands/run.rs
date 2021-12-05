use std::error::Error;
use std::thread;
use std::time::Duration;

use crate::config::WallpapersConfig;
use crate::background;

pub fn run() -> Result<(), Box<dyn Error>> {
    let config = WallpapersConfig::load()?;
    
    let timer = thread::spawn(move || {
        loop {
            let config = config.clone();
            let duration = config.duration;
            thread::sleep(Duration::from_secs(duration));
            
            let _ = background::change(config);
        }
    });

    timer
        .join()
        .expect("Failed to start background timer");

    Ok(())
}