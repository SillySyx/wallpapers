use std::process::Command;

#[derive(Debug, PartialEq)]
pub enum ColorScheme {
    Light,
    Dark,
}

pub fn get_color_scheme(override_color_scheme: &Option<String>) -> Option<ColorScheme> {
    if let Some(color_scheme) = override_color_scheme {
        return Some(parse_color_scheme(color_scheme));
    }

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

    Some(parse_color_scheme(&color_scheme))
}

fn parse_color_scheme(color_scheme: &str) -> ColorScheme {
    match color_scheme {
        "'prefer-dark'" => ColorScheme::Dark,
        "prefer-dark" => ColorScheme::Dark,
        _ => ColorScheme::Light,
    }
}

#[cfg(test)]
mod tests {
    use super::get_color_scheme;

    #[test]
    fn should_be_able_to_read_color_scheme() {
        let result = get_color_scheme(&None);

        assert_ne!(None, result);
    }
}