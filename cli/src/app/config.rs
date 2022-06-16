use figment::providers::{Format, Serialized, Yaml};
use figment::Figment;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    key: String,
    value: i32,
}

impl Config {
    pub fn parse(path: PathBuf) -> figment::error::Result<Config> {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Yaml::file(path))
            .extract()
    }
}
