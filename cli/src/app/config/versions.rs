use anonymizer_lib::PatientSex;
use anyhow::Result;
use dicom_core::value::DicomDateTime;
use dicom_core::Tag;
use std::fmt::Formatter;
use std::str::FromStr;
use yaml_rust::Yaml;

use crate::app::types::IConfigFile;
use crate::app::utils::{parse_datetime_utc, parse_tag};
use crate::generate_key;

// TODO switch from core::Option to a self-made enum with None, Remove and Change (also in over cli!)
#[derive(Default, Debug)]
pub struct ConfigFileV1 {
    pub patient_name: Option<String>,
    pub patient_birth_day: Option<DicomDateTime>,
    pub patient_sex: Option<PatientSex>,
    pub remove_tags: Option<Vec<Tag>>,
}

impl IConfigFile for ConfigFileV1 {
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

        Ok(Box::from(Self {
            patient_name,
            patient_birth_day,
            patient_sex,
            remove_tags,
        }))
    }

    fn get_version() -> String {
        "1.0".to_string()
    }

    fn pretty_print(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO make better and more beautiful
        write!(f, "Patient:\n")?;
        write!(f, "\tName: {:?}\n", self.patient_name)?;
        write!(f, "\tBirthday: {:?}\n", self.patient_birth_day)?;
        write!(f, "\tSex: {:?}\n", self.patient_sex)?;
        write!(f, "\nRemove Tags: {:?}\n", self.remove_tags)
    }
}
