use anonymizer_lib::PatientSex;
use anyhow::Result;
use dicom_core::value::DicomDateTime;
use dicom_core::Tag;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use yaml_rust::{Yaml, YamlLoader};

use crate::app::types::IConfigFile;
use crate::app::utils::{parse_datetime_utc, parse_tag};
use crate::generate_key;

#[derive(Default, Debug)]
pub struct ConfigFile {
    pub patient_name: Option<String>,
    pub patient_birth_day: Option<DicomDateTime>,
    pub patient_sex: Option<PatientSex>,
    pub remove_tags: Option<Vec<Tag>>,
}

impl IConfigFile for ConfigFile {
    fn parse(content: Vec<Yaml>) -> Result<Box<Self>> {
        let content = content
            .get(0)
            .expect("Can't get first element in yaml-tree");
        let content = match content {
            Yaml::Hash(value) => Some(value),
            _ => None,
        }
        .expect("Should be a hash");
        let content = content
            .get(&Yaml::String("config".to_string()))
            .expect("Should have a config entry");
        let content = match content {
            Yaml::Hash(value) => Some(value),
            _ => None,
        }
        .expect("Should be a hash");

        let patient_name = content
            .get(generate_key!("patient_name"))
            .map(|v| v.as_str().expect("Has to be a string").to_string());
        let patient_birth_day = content.get(generate_key!("patient_birth_day")).map(|v| {
            let date_raw = v.as_str().expect("Has to be a string");
            let dt_offset = parse_datetime_utc(date_raw).expect("Error while parsing birth day");
            DicomDateTime::try_from(&dt_offset).expect("Transform into DicomDateTime")
        });
        let patient_sex = content.get(generate_key!("patient_sex")).map(|v| {
            PatientSex::from_str(v.as_str().expect("Has to be a string"))
                .expect("Value must to be M, F or O")
        });
        let remove_tags = content.get(generate_key!("remove_tags")).map(|values| {
            let tags_raw = values.as_vec().expect("Should be an list of tags");
            tags_raw
                .iter()
                .map(|v| {
                    let tag_raw = v
                        .as_str()
                        .expect("Raw tag value has to be a string")
                        .to_string();

                    parse_tag(&tag_raw).expect("Error while parsing tag")
                })
                .collect::<Vec<Tag>>()
        });

        Ok(Box::from(ConfigFile {
            patient_name,
            patient_birth_day,
            patient_sex,
            remove_tags,
        }))
    }

    fn get_version() -> String {
        "1.0".to_string()
    }
}

#[derive(Debug)]
pub enum ConfigFileVersions {
    V1_0(ConfigFile),
}

impl ConfigFileVersions {
    pub fn parse(path: PathBuf) -> Option<Self> {
        let mut file = File::open(path).ok()?;
        let mut content = String::new();
        file.read_to_string(&mut content).ok()?;

        let parsed = YamlLoader::load_from_str(&content).ok()?;

        match parsed[0]["version"].as_str().unwrap_or("1.0") {
            "1.0" => {
                let cf = *ConfigFile::parse(parsed).ok()?;

                Some(ConfigFileVersions::V1_0(cf))
            }
            _ => None,
        }
    }
}
