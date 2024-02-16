use super::super::process_stdout::process_stdout;
use std::process::Command;
use std::process::Stdio;

pub fn custom(command: &str, args: &Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;
    // .output()
    // .expect("failed to execute process");

    match process_stdout(&mut child) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}
