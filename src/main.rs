use std::env;

mod lsm;
mod lsm_core;
mod lsm_registry;

mod global_data;
use global_data::GlobalData;
use global_data::GlobalDataTrait;

fn main() {
    // Detect OS and declare the target path
    let glob: GlobalData = match env::consts::OS {
        "macos" => GlobalData::new("~/.local/lsm/".to_string(), "~/.local/bin/".to_string()),
        "linux" => GlobalData::new("./.local/lsm/".to_string(), "./.local/bin/".to_string()),
        "windows" => GlobalData::new("./.local/lsm/".to_string(), "./.local/bin/".to_string()),
        _ => panic!("Unsupported OS"),
    };

    // let file = match std::fs::File::open("./target/tmp.yaml") {
    //     Ok(file) => file,
    //     Err(e) => panic!("{e}"),
    // };
    // let reader = std::io::BufReader::new(file);
    // let deserialized_map: lsm_registry::RegistryStruct = match serde_yaml::from_reader(reader) {
    //     Ok(s) => s,
    //     Err(e) => panic!("{e}"),
    // };
    // println!("{:?}", deserialized_map);
    // return;

    // Parse arguments
    let opt = lsm::args::get_options();
    println!("opt: {:?}", opt);

    // List all registry
    if opt.list {
        lsm_core::list(&glob);
    }

    // Handle install request
    match opt.package {
        Some(p) => lsm_core::install(&glob, &p),
        None => println!("No package to install"),
    }
}
