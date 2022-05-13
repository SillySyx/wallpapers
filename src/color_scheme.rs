use std::process::Command;

#[derive(Debug, PartialEq)]
pub enum ColorScheme {
    Light,
    Dark,
}

pub fn get_color_scheme() -> Option<ColorScheme> {
    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "color-scheme"])
        .output();

    let output = match output {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    let color_scheme = match String::from_utf8(output.stdout) {
        Ok(value) => value.trim().to_string(),
        Err(_) => return None,
    };

    if color_scheme == "'prefer-dark'" {
        return Some(ColorScheme::Dark);
    }

    Some(ColorScheme::Light)
}

#[cfg(test)]
mod tests {
    use super::get_color_scheme;

    #[test]
    fn should_be_able_to_read_color_scheme() {
        let result = get_color_scheme();

        assert_ne!(None, result);
    }
}