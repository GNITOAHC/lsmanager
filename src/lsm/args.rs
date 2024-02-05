// use std::path::PathBuf;
// use std::fmt;
use structopt::StructOpt;

// impl fmt::Display for Opt {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.package)
//     }
// }

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    #[structopt(short = "i", long = "install")]
    pub package: Option<String>,

    #[structopt(short = "u", long = "uninstall")]
    pub uninstalled_package: Option<String>,

    #[structopt(short = "l", long = "list")]
    pub list: bool,

    #[structopt(short = "U", long = "update")]
    pub update: bool,
}

pub fn get_options() -> Opt {
    Opt::from_args()
}
