extern crate yaml_rust;

mod app;

use app::cli::App;

fn main() {
    App::run();
}
