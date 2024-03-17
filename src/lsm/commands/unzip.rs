use crate::lsm::process_stdout::process_stdout;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

fn unzip_unzip(
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
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn unzip_gzip(zip_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // gzip -d [file]
    // e.g. gzip -d rust-analyzer/rust-analyzer-aarch64-apple-darwin.gz
    let mut child = Command::new("gzip")
        .arg("-d")
        .arg(zip_file_path)
        .stdout(Stdio::piped())
        .spawn()?;
    match process_stdout(&mut child) {
        Ok(_) => chmod(&zip_file_path.replace(".gz", ""), "+x"),
        Err(e) => Err(e),
    }
}

pub fn unzip(
    destination: &Path,
    zip_file_path: &str,
    extract_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match zip_file_path.split('.').last() {
        Some("zip") => unzip_unzip(destination, zip_file_path, extract_path),
        Some("gz") => unzip_gzip(zip_file_path),
        _ => panic!("Unsupported file type"),
    }
}

fn chmod(file_path: &str, flag: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("chmod")
        .arg(flag)
        .arg(file_path)
        .stdout(Stdio::piped())
        .spawn()?;

    match process_stdout(&mut child) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
