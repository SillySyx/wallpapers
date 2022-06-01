use std::error::Error;
use crate::home_folder::resolve_home_folder;

#[derive(Clone, Debug)]
pub struct WallpapersConfig {
    pub duration: u64,
    pub image_folder: String,
    pub override_color_scheme: Option<String>,
}

impl WallpapersConfig {
    pub fn default() -> Result<Self, Box<dyn Error>> {
        let image_folder = resolve_home_folder("~/Pictures");

        Ok(Self {
            duration: 1800,
            image_folder,
            override_color_scheme: None,
        })
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = resolve_home_folder("~/.config/wallpapers/settings");

        let data = std::fs::read(config_path)?;
        let content = String::from_utf8(data)?;

        let mut config = WallpapersConfig::default()?;

        for line in content.lines() {
            if let Some((name, value)) = parse_config_value(line.to_string()) {
                if name == "Duration" {
                    config.duration = value.parse::<u64>()?;
                }
                if name == "Image folder" {
                    config.image_folder = resolve_home_folder(value.clone());
                }
                if name == "Override color scheme" {
                    config.override_color_scheme = Some(value.clone());
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
