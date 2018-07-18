use failure::Error;
use std::process::Command;

pub fn execute(cmd: &[&str]) -> Result<String, Error> {
    let output = Command::new("vcgencmd")
        .args(cmd)
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}