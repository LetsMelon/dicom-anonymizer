use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use strum::EnumCount;
use yaml_rust::YamlLoader;

use crate::app::config::versions::ConfigFileV1;
use crate::app::types::IConfigFile;

#[derive(Debug, EnumCount)]
pub enum ConfigFileVersions {
    V1_0(ConfigFileV1),
}

impl ConfigFileVersions {
    pub fn parse(path: PathBuf) -> Option<Self> {
        let mut file = File::open(path).ok()?;
        let mut content = String::new();
        file.read_to_string(&mut content).ok()?;

        let parsed = YamlLoader::load_from_str(&content).ok()?;

        debug_assert!(
            ConfigFileVersions::COUNT == 1,
            "TODO: implement new 'ConfigFileVersions' member in match statement"
        );
        match parsed[0]["version"].as_str().unwrap_or("1.0") {
            "1.0" => {
                let cf = *ConfigFileV1::parse(parsed).ok()?;

                Some(ConfigFileVersions::V1_0(cf))
            }
            _ => None,
        }
    }
}

impl Display for ConfigFileVersions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigFileVersions::V1_0(file) => file.pretty_print(f),
        }
    }
}
