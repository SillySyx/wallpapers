use std::error::Error;

use super::user::get_current_user;

#[derive(Clone, Debug)]
pub struct WallpapersConfig {
    pub duration: u64,
    pub image_folder: String,
}

impl WallpapersConfig {
    pub fn default() -> Result<Self, Box<dyn Error>> {
        let user = get_current_user()?;
        let image_folder = format!("/home/{}/Pictures", user);

        Ok(Self {
            duration: 1800,
            image_folder,
        })
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let user = get_current_user()?;
        let config_path = format!("/home/{}/.config/wallpapers/settings", user);

        let data = std::fs::read(config_path)?;
        let content = String::from_utf8(data)?;

        let mut config = WallpapersConfig::default()?;

        for line in content.lines() {
            if let Some((name, value)) = parse_config_value(line.to_string()) {
                if name == "Duration" {
                    config.duration = value.parse::<u64>()?;
                }
                if name == "Image folder" {
                    config.image_folder = value;
                }
            }
        }

        Ok(config)
    }
}

fn parse_config_value(config_value: String) -> Option<(String, String)> {
    if config_value.contains("=") {
        let values: Vec<String> = config_value
            .split("=")
            .into_iter()
            .map(|value| value.to_string())
            .collect();

        let value = (values.first()?.to_owned(), values.last()?.to_owned());

        return Some(value);
    }

    None
}
