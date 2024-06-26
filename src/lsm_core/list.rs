// use crate::lsm_registry;
use crate::global_data::GlobalDataTrait;
use crate::GlobalData;
use std::fs;
use std::path::Path;

fn list_dir(path: &Path) -> () {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let display: String = entry.path().into_os_string().into_string().unwrap();
        println!("{}", display.split("/").last().unwrap())
    }
}

pub fn list(glob: &GlobalData) -> () {
    match Path::new(&glob.get_dir_path()).exists() {
        // Walk through all installed packages
        true => {
            if let false = Path::new(&[&glob.get_dir_path(), "/packages"].join("")).exists() {
                println!("No packages installed");
                return;
            }
            println!("Installed packages: (located in {}/packages)", &glob.get_dir_path());
            list_dir(Path::new(&[&glob.get_dir_path(), "/packages"].join("")))
        }
        // Create new directory if not exists
        false => {
            if let Err(e) = fs::create_dir_all(&glob.get_dir_path()) {
                println!("Error creating {}", &glob.get_dir_path());
                println!("{e}")
            }
        }
    }
}
