use std::fmt;

pub struct GlobalData {
    dir_path: String,
    bin_path: String,
    os: String,
    arch: String,
}

impl fmt::Display for GlobalData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.dir_path)
    }
}

pub enum OsType {
    DarwinX64,
    DarwinArm64,
    LinuxX64,
    LinuxArm64,
    WindowsX64,
    WindowsArm64,
}

pub trait GlobalDataTrait {
    fn new() -> Self;
    fn get_dir_path(&self) -> &str;
    fn get_pkg_path(&self, pkg: &str) -> String;
    fn get_bin_path(&self) -> &str;
    fn get_os(&self) -> &str;
    fn get_os_string(&self) -> String;
    fn get_arch(&self) -> &str;
    fn get_os_type(&self) -> OsType;
}

fn get_destination() -> (String, String) {
    match std::env::consts::OS {
        "macos" => ("~/.local/lsm/".to_string(), "~/.local/bin/".to_string()),
        "linux" => ("./.local/lsm/".to_string(), "./.local/bin/".to_string()),
        "windows" => ("./.local/lsm/".to_string(), "./.local/bin/".to_string()),
        _ => panic!("Unsupported OS"),
    }
}

fn get_home() -> Result<String, std::env::VarError> {
    match std::env::consts::OS {
        "windows" => std::env::var("HOMEPATH"),
        _ => std::env::var("HOME"),
    }
}

impl GlobalDataTrait for GlobalData {
    fn new() -> Self {
        let (dir, bin) = get_destination();
        let home = match get_home() {
            Ok(h) => h,
            Err(e) => panic!("Error {}", e),
        };
        GlobalData {
            dir_path: dir.replace("~", &home),
            bin_path: bin.replace("~", &home),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        }
    }

    fn get_dir_path(&self) -> &str {
        &self.dir_path
    }

    fn get_pkg_path(&self, pkg: &str) -> String {
        format!("{}packages/{}/", self.dir_path, pkg)
    }

    fn get_bin_path(&self) -> &str {
        &self.bin_path
    }

    fn get_os(&self) -> &str {
        &self.os
    }

    fn get_os_string(&self) -> String {
        match self.get_os_type() {
            OsType::DarwinX64 => "darwin_x64".to_string(),
            OsType::DarwinArm64 => "darwin_arm64".to_string(),
            OsType::LinuxX64 => "linux_x64".to_string(),
            OsType::LinuxArm64 => "linux_arm64".to_string(),
            OsType::WindowsX64 => "win_x64".to_string(),
            OsType::WindowsArm64 => "win_arm64".to_string(),
        }
    }

    fn get_arch(&self) -> &str {
        &self.arch
    }

    fn get_os_type(&self) -> OsType {
        match self.os.as_str() {
            "macos" => match self.arch.as_str() {
                "x86_64" => OsType::DarwinX64,
                "aarch64" => OsType::DarwinArm64,
                _ => panic!("Unsupported architecture"),
            },
            "linux" => match self.arch.as_str() {
                "x86_64" => OsType::LinuxX64,
                "aarch64" => OsType::LinuxArm64,
                _ => panic!("Unsupported architecture"),
            },
            "windows" => match self.arch.as_str() {
                "x86_64" => OsType::WindowsX64,
                "aarch64" => OsType::WindowsArm64,
                _ => panic!("Unsupported architecture"),
            },
            _ => panic!("Unsupported OS"),
        }
    }
}
