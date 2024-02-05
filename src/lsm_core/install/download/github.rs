use crate::global_data::GlobalDataTrait;
use crate::lsm_registry;

// Get (source, version) from source string
pub fn get_details(source: &str, prefix: &str) -> (String, String) {
    (
        source.replace(prefix, "").split("@").collect::<Vec<&str>>()[0].to_string(),
        source.split("@").collect::<Vec<&str>>()[1].to_string(),
    )
}

// ::return:: bin path in zip file
pub fn get_bin_path(map: &lsm_registry::RegistryStruct, os_string: &str) -> String {
    let bin_path: String;
    let version = get_details(&map.source.id, "pkg:github/").1;
    let assets = match &map.source.asset {
        Some(x) => x,
        None => panic!("No asset found"),
    };
    match assets.iter().find(|&x| x.target == os_string) {
        Some(x) => bin_path = x.bin.to_string(),
        None => return "not_found".to_string(),
    }
    bin_path.replace("{{version}}", version.as_str())
}

// Generate download url and zipname
pub fn handle_github(
    raw_source: &str,
    glob: &crate::GlobalData,
    map: &lsm_registry::RegistryStruct,
) -> (String, String) {
    println!("handle_github");
    let (source, version) = get_details(raw_source, "pkg:github/");
    let os_string = glob.get_os_string();
    let assets = match &map.source.asset {
        Some(x) => x,
        None => panic!("No asset found"),
    };
    let mut zipname: String = match assets.iter().find(|&x| x.target == os_string) {
        Some(x) => x.file.to_string(),
        None => "not_found".to_string(),
    };
    zipname = zipname.replace("{{version}}", &version);

    // Generate download url
    let download_url = format!(
        "https://github.com/{}/releases/download/{}/{}",
        source, version, zipname
    );
    println!("{}", download_url);
    return (download_url, zipname);
}
