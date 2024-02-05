use super::download::download;
use crate::global_data::GlobalDataTrait;
use crate::lsm::commands::curl;
use crate::lsm::commands::symlink::symlink;
use crate::lsm_registry;
use crate::GlobalData;
use std::fs;
use std::path::Path;

/*
 *  1. Make sure package/info.yaml exists in packages/{lang}
 *    - If not, fetch info.yaml into packages/{lang}/info.yaml
 *  2. Read info.yaml
 *  3. Download dependencies (including download executable from package managers)
 *  4. Extract binary (make it executable) and link to bin (change name to package name)
 */

pub fn install(glob: &GlobalData, package: &str) {
    let pkg_string = &glob.get_pkg_path(package);
    let package_path = Path::new(&pkg_string);
    let info_path = [pkg_string, "info.yaml"].join("");

    // 1. Make sure package/info.yaml exists in packages/{lang}
    match package_path.exists() {
        true => println!("package exists"), // Package exists, return
        false => {
            // - If not, fetch info.yaml into packages/{lang}/info.yaml
            println!("package not exists");
            // Create package directory
            if let Err(e) = fs::create_dir_all(&glob.get_pkg_path(package)) {
                println!("Error creating {}", &glob.get_pkg_path(package),);
                println!("{e}")
            }
            // Fetch info.yaml into packages/{lang}/info.yaml
            let download_url = format!(
                "https://raw.githubusercontent.com/GNITOAHC/lsm_packages/main/packages/{}/info.yaml",
                package
            );
            match curl::curl(&download_url, &info_path) {
                Ok(_) => println!("fetched info.yaml into {}", info_path),
                Err(e) => panic!("{e}"),
            }
        }
    }

    // 2. Read info.yaml
    let file = match std::fs::File::open(info_path) {
        Ok(file) => file,
        Err(e) => panic!("{e}"),
    };
    let reader = std::io::BufReader::new(file);
    let deserialized_map: lsm_registry::RegistryStruct = match serde_yaml::from_reader(reader) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };
    // println!("{:?}", deserialized_map);

    // 3. Download dependencies (including download executable from package managers)
    // ::return:: executable name
    let bin_name = match download(
        &deserialized_map, // Contains info.yaml
        &glob,
        package_path, // Path to package
    ) {
        Ok(name) => name,
        Err(e) => panic!("{e}"),
    };

    // Make sure bin directory exists
    if let Err(e) = fs::create_dir_all(&glob.get_bin_path()) {
        println!("Error creating {}", &glob.get_bin_path());
        println!("{e}")
    }

    // 4. Extract binary (make it executable) and link to bin (change name to package name)
    match symlink(
        [&glob.get_pkg_path(package), bin_name.as_str()].join(""),
        [&glob.get_bin_path(), deserialized_map.name.as_str()].join(""),
    ) {
        Ok(_) => println!("linked"),
        Err(e) => println!("{e}"),
    }
}
