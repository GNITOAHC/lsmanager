use crate::lsm::commands;
use std::collections::{hash_map::RandomState, HashMap};

fn npm_install(prefix: &str, package: &str) -> Result<(), Box<dyn std::error::Error>> {
    let command = "npm";
    let args = vec!["install", "--prefix", prefix, package];
    match commands::custom::custom(command, &args) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

// ::return:: version
pub fn get_package(source: &str) -> String {
    // e.g. prettier@3.2.5 or @tailwindcss/language-server@0.0.16
    match source.split("pkg:npm/").last() {
        Some(x) => x.replace("%40", "@"), // prevent format like %40tailwindcss/language-server@0.0.16
        None => panic!("No version found"),
    }
}

pub fn handle_npm(
    pkg: &str,
    pkg_path: &str,
    bins: &Option<HashMap<String, String, RandomState>>,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    // e.g. prettier@3.2.5
    if let Err(e) = npm_install(pkg_path, &pkg) {
        return Err(e);
    }

    let pkg_trans = |s: &String| -> String {
        return format!("./node_modules/.bin/{}", s.replace("npm:", "").trim());
    };

    match bins {
        Some(bins) => {
            let bin_path_map: HashMap<String, String> = bins
                .iter()
                .map(|(key, val)| {
                    // HashMap<src, dst>
                    return (pkg_trans(val).clone(), key.clone());
                })
                .collect();
            return Ok(bin_path_map);
        }
        None => {
            return Err("No bin found".into());
        }
    }
}
