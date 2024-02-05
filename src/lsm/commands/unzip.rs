use crate::lsm::process_stdout::process_stdout;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

pub fn unzip(
    destination: &Path,
    zip_file_path: &str,
    extract_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // [-d exdir] extract files into exdir
    // [-j] junk paths. The archive's directory structure is not recreated; all files are deposited in the extraction directory (by default, the current one).
    // unzip -j -d [exdir] [zipfile] [file to extract]
    // e.g.
    // unzip -j -d ./.local/lsm/packages/clangd/ ./.local/lsm/packages/clangd/clangd-mac-16.0.2.zip clangd_16.0.2/bin/clangd
    let mut child = Command::new("unzip")
        .arg("-j")
        .arg("-d")
        .arg(destination)
        .arg(zip_file_path)
        .arg(extract_path)
        .stdout(Stdio::piped())
        .spawn()?;

    match process_stdout(&mut child) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}
