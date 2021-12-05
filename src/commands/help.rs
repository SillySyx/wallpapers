use std::error::Error;

pub fn help() -> Result<(), Box<dyn Error>> {
    println!("COMMANDS");
    println!("\trun\t\tStart wallpaper slideshow, changes images every time timer runs out.");
    println!("\tnext\t\tChanges wallpaper to a random image");

    Ok(())
}