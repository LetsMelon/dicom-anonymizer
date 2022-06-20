extern crate yaml_rust;

mod app;

use crate::app::config::file::ConfigFileVersions;
use app::cli::App;
use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new("./test.yaml");
    let config = ConfigFileVersions::parse(PathBuf::from(path));
    println!("{:?}", config);

    App::run();
}
