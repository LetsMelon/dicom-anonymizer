use crate::app::config::ConfigFileVersions;
use anonymizer_lib::types::CustomDicomDateTime;
use anonymizer_lib::{Anonymizer, AnonymizerMeta, PatientSex, TagAction};
use anyhow::Result;
use clap::ArgMatches;
use dicom_core::value::DicomDateTime;
use dicom_core::Tag;
use std::path::PathBuf;
use std::str::FromStr;

use crate::app::types::IMatcher;
use crate::app::utils::{parse_datetime_utc, parse_tag};

#[derive(Debug)]
pub struct AnonymizerValues {
    pub(crate) input: PathBuf,
    pub(crate) output: Option<PathBuf>,
    pub(crate) patient_name: TagAction<String>,
    pub(crate) patient_sex: TagAction<PatientSex>,
    pub(crate) patient_birth_day: TagAction<DicomDateTime>,
    pub(crate) remove_tags: Vec<Tag>,
    pub(crate) dry_run: bool,
}

impl IMatcher<AnonymizerMeta> for AnonymizerValues {
    fn match_args(matches: ArgMatches) -> Result<Box<AnonymizerValues>> {
        let dry_run = match matches.value_of("dry_run") {
            None => false,
            Some(value) => value.parse().unwrap_or(false),
        };

        let input = PathBuf::from(matches.value_of("input").unwrap());
        let output = matches.value_of("output").map(PathBuf::from);

        let mut patient_name =
            TagAction::from(matches.value_of("patient_name").map(str::to_string));
        let mut patient_sex = match matches.value_of("patient_sex") {
            None => TagAction::Keep,
            Some(v) => TagAction::Change(PatientSex::from_str(v)?),
        };
        let mut patient_birth_day = match matches.value_of("patient_birth_day") {
            None => TagAction::Keep,
            Some(pbd) => {
                let dt_offset = parse_datetime_utc(pbd)?;
                TagAction::Change(DicomDateTime::try_from(&dt_offset)?)
            }
        };
        let mut remove_tags = match matches.values_of("remove_tags") {
            None => Vec::new(),
            Some(rt) => {
                let mut remove_tags = Vec::<Tag>::new();
                for item in rt {
                    remove_tags.push(parse_tag(item)?);
                }

                remove_tags
            }
        };

        match matches.value_of("config").map(PathBuf::from) {
            None => (),
            Some(p) => {
                let cfv = ConfigFileVersions::parse(p).unwrap();

                println!("{:?}", cfv);

                match cfv {
                    ConfigFileVersions::V1_0(data) => {
                        match patient_name {
                            TagAction::Change(data) => {
                                patient_name = TagAction::Change(data);
                            }
                            _ => (),
                        }

                        match patient_birth_day {
                            TagAction::Change(data) => {
                                patient_birth_day = TagAction::Change(data);
                            }
                            _ => (),
                        }

                        match patient_sex {
                            TagAction::Change(data) => {
                                patient_sex = TagAction::Change(data);
                            }
                            _ => (),
                        }

                        match data.remove_tags {
                            None => (),
                            Some(data) => {
                                remove_tags.extend(data);
                            }
                        };
                    }
                    ConfigFileVersions::V1_1(data) => {
                        match patient_name {
                            TagAction::Change(data) => {
                                patient_name = TagAction::Change(data);
                            }
                            _ => (),
                        }

                        match patient_birth_day {
                            TagAction::Change(data) => {
                                patient_birth_day = TagAction::Change(data);
                            }
                            _ => (),
                        }

                        match patient_sex {
                            TagAction::Change(data) => {
                                patient_sex = TagAction::Change(data);
                            }
                            _ => (),
                        }

                        remove_tags.extend(data.remove_tags);
                    }
                }
            }
        };

        Ok(Box::from(AnonymizerValues {
            input,
            output,
            patient_name,
            patient_sex,
            patient_birth_day,
            remove_tags,
            dry_run,
        }))
    }

    fn match_trait(&self) -> Result<AnonymizerMeta> {
        let mut builder = Anonymizer::meta_builder();

        builder.patient_name(self.patient_name.clone());
        builder.patient_sex(self.patient_sex.clone());
        let cddt_tag_action = self
            .patient_birth_day
            .clone()
            .map(|value| CustomDicomDateTime::from(value));
        builder.patient_birth_date(cddt_tag_action);
        builder.remove_tags(self.remove_tags.to_owned().into());

        Ok(builder.build()?)
    }
}
