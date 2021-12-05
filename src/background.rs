use std::error::Error;
use std::process::Command;
use std::fs::read_dir;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::config::WallpapersConfig;

pub fn change(config: WallpapersConfig) -> Result<(), Box<dyn Error>> {
    let images = get_images_in_folder(&config.image_folder)?;
    let number_of_images = images.len();
    
    let image = select_random_image(images)?;
    let current_image = get_background_image()?;
    
    if image == current_image && number_of_images > 1 {
        return change(config);
    } 

    set_background_image(config.image_folder, image)
}

fn set_background_image(image_folder: String, image: String) -> Result<(), Box<dyn Error>> {
    // gsettings set org.gnome.desktop.background picture-uri file:///path/to/your/image.png

    let image_path = format!("file://{}/{}", image_folder, image);

    let _ = Command::new("gsettings")
        .args(["set", "org.gnome.desktop.background", "picture-uri", &image_path])
        .output()?;

    Ok(())
}

fn get_background_image() -> Result<String, Box<dyn Error>> {
    // gsettings get org.gnome.desktop.background picture-uri

    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.background", "picture-uri"])
        .output()?;

    let image_path = String::from_utf8(output.stdout)?;

    if let Some(index) = image_path.rfind('/') {
        let image = image_path[(index + 1)..(image_path.len() - 2)].to_string();
        return Ok(image);
    }

    Ok(image_path)
}

fn get_images_in_folder(image_folder_path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut images = vec![];

    for entry in read_dir(image_folder_path)? {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                continue;
            }

            if let Some(name) = entry.file_name().to_str() {
                images.push(name.to_string());
            }
        }
    }

    Ok(images)
}

fn select_random_image(images: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut rng = thread_rng();

    let mut images = images;
    images.shuffle(&mut rng);

    let image = images.first().unwrap();

    Ok(image.to_owned())
}
