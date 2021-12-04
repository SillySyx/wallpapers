use std::error::Error;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::fs::read_dir;
use rand::seq::SliceRandom;
use rand::thread_rng;

// Tray icon
// https://github.com/iovxw/ksni
// https://github.com/olback/tray-item-rs

fn main() -> Result<(), Box<dyn Error>> {
    let timer = thread::spawn(|| {
        loop {
            let sleep_duration = 30 * 60;
            thread::sleep(Duration::from_secs(sleep_duration));
            change_background().expect("Failed to change background");
        }
    });

    timer.join().expect("Failed to schedule timer");
    Ok(())
}

fn change_background() -> Result<(), Box<dyn Error>> {
    // gsettings set org.gnome.desktop.background picture-uri file:///path/to/your/image.png

    let image_folder_path = get_images_folder()?;
    let images = get_images(&image_folder_path)?;
    let image = select_image(images)?;

    let image_path = format!("file://{}/{}", image_folder_path, image);

    Command::new("gsettings")
        .args(["set", "org.gnome.desktop.background", "picture-uri", &image_path])
        .output()?;

    Ok(())
}

fn get_images_folder() -> Result<String, Box<dyn Error>> {
    Ok("/home/syx/Pictures/wallies5".to_string())
    // Ok("~/Pictures".to_string())
}

fn get_images(image_folder_path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut images = vec![];

    for entry in read_dir(image_folder_path)? {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                images.push(name.to_string());
            }
        }
    }

    Ok(images)
}

fn select_image(images: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut rng = thread_rng();

    let mut images = images;
    images.shuffle(&mut rng);

    let image = images.first().unwrap();

    Ok(image.to_owned())
}