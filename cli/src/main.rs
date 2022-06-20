extern crate yaml_rust;

mod app;

use crate::app::config::file::ConfigFileVersions;
use app::cli::App;

fn main() {
    App::run();
}
