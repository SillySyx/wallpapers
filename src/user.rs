use std::error::Error;
use std::process::Command;
use std::process::Output;

pub fn get_current_user() -> Result<String, Box<dyn Error>> {
    let output = Command::new("whoami").output()?;

    let values = parse_output_to_strings(output)?;

    let user = values.first().unwrap().to_owned();

    Ok(user)
}

fn parse_output_to_strings(output: Output) -> Result<Vec<String>, Box<dyn Error>> {
    let value = String::from_utf8(output.stdout)?;

    let strings: Vec<String> = value
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    Ok(strings)
}