use super::{github, npm, pypi};
use crate::global_data::GlobalDataTrait;
use crate::lsm::commands::{curl::curl, unzip::unzip};
use crate::lsm_registry;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
enum PackageSource {
    Undefined,
    Unknown,
    Github,
    Npm,
    Pypi,
}

// Download dependencies and return filename of binary
pub fn download(
    map: &lsm_registry::RegistryStruct,
    glob: &crate::GlobalData,
    path: &Path, // Path to `lsm/packages/` directory
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let raw_source = &map.source.id;
    let mut package_source: PackageSource = PackageSource::Undefined;

    ["pkg:github", "pkg:npm", "pkg:pypi"]
        .iter()
        .for_each(|prefix| {
            if !raw_source.starts_with(prefix) {
                return;
            }
            match prefix {
                &"pkg:github" => package_source = PackageSource::Github,
                &"pkg:npm" => package_source = PackageSource::Npm,
                &"pkg:pypi" => package_source = PackageSource::Pypi,
                _ => package_source = PackageSource::Unknown,
            }
        });

    match package_source {
        PackageSource::Github => {
            let (download_url, zipname) = github::handle_github(&raw_source, &glob, &map);
            let bin_path = github::get_bin_path(&map, &glob.get_os_string()); // bin path in zip file
            let destination = format!("{}/{}", path.to_string_lossy(), zipname); // path to save zip file

            // Download zip file
            if let Err(e) = curl(&download_url, &destination) {
                return Err(e);
            }
            // Extract binary from zip file: unzip(destination, zip_file_path, extract_path)
            if let Err(e) = unzip(&path, path.join(&zipname).to_str().unwrap(), &bin_path) {
                return Err(e);
            }
            // Return the filename of binary
            // HashMap<src, dst>
            let bin_path_map = HashMap::from([(
                bin_path.split('/').last().unwrap().to_string(), // src
                map.name.clone(),                                // dst
            )]);
            return Ok(bin_path_map);
        }
        PackageSource::Npm => {
            let package = npm::get_package(&map.source.id); // e.g. prettier@3.2.5 or @tailwindcss/language-server@0.0.16
            let dir = glob.get_pkg_path(&map.name); // Path to `lsm/packages/{lang}/`
            match npm::handle_npm(&package, &dir, &map.bin) {
                Ok(bin_path_map) => return Ok(bin_path_map),
                Err(e) => return Err(e),
            }
        }
        PackageSource::Pypi => {
            pypi::handle_pypi(&map.source.id);
        }
        _ => return Err("Unknown package source".into()),
    }
    Ok(HashMap::from([]))
}
