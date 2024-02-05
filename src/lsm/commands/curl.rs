use super::super::process_stdout::process_stdout;
use std::process::Command;
use std::process::Stdio;

pub fn curl(download_url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("curl")
        .arg("-L")
        .arg(download_url)
        .arg("-o")
        .arg(destination)
        .arg("-#")
        .stdout(Stdio::piped())
        .spawn()?;
    // .output()
    // .expect("failed to execute process");

    match process_stdout(&mut child) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}
