use std::error::Error;
use std::process::Command;
use std::fs::read_dir;

use crate::config::WallpapersConfig;
use crate::color_scheme::{get_color_scheme, ColorScheme};

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

    let picture_uri = match get_color_scheme() {
        Some(ColorScheme::Dark) => "picture-uri-dark",
        Some(ColorScheme::Light) => "picture-uri",
        _ => "picture-uri",
    };

    let image_path = format!("file://{}/{}", image_folder, image);

    let _ = Command::new("gsettings")
        .args(["set", "org.gnome.desktop.background", picture_uri, &image_path])
        .output()?;

    Ok(())
}

fn get_background_image() -> Result<String, Box<dyn Error>> {
    // gsettings get org.gnome.desktop.background picture-uri

    let picture_uri = match get_color_scheme() {
        Some(ColorScheme::Dark) => "picture-uri-dark",
        Some(ColorScheme::Light) => "picture-uri",
        _ => "picture-uri",
    };

    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.background", picture_uri])
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

fn random_number(min: usize, max: usize) -> Result<usize, Box<dyn Error>> {
    let range = format!("{}-{}", min, max);

    let output = Command::new("shuf")
        .args(["-i", &range, "-n1"])
        .output()?;

    let value = String::from_utf8(output.stdout)?;
    let number = value.trim().parse::<usize>()?;

    Ok(number)
}

fn select_random_image(images: Vec<String>) -> Result<String, Box<dyn Error>> {
    let index = random_number(0, images.len())?;

    if let Some(image) = images.get(index) {
        return Ok(image.to_owned());
    }

    Err(Box::from("Failed to get random image"))
}
