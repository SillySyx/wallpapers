use std::env::var_os;

pub fn resolve_home_folder<T: Into<String>>(path: T) -> String {
    let path = path.into();
    
    if !path.starts_with("~/") {
        return path;
    }

    let home = match read_home_from_environment_variables() {
        Some(value) => value,
        None => return path,
    };

    format!("{}/{}", home, &path[2..])
}

pub fn read_home_from_environment_variables() -> Option<String> {
    let value = var_os("HOME")?;
    let value = value.to_str()?;

    Some(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_read_home_from_environment_variables() {
        if let Some(home) = read_home_from_environment_variables() {
            assert_ne!("", home);
            return;
        }

        panic!("oh no!");
    }

    #[test]
    fn should_be_able_to_resolve_home_folder() {
        let path = "~/p/test";
        let result = resolve_home_folder(path);
        assert_ne!("~/p/test", result);
    }
}