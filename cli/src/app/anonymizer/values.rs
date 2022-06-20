use anonymizer_lib::types::CustomDicomDateTime;
use anonymizer_lib::{Anonymizer, AnonymizerMeta, PatientSex};
use anyhow::Result;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, Utc};
use clap::ArgMatches;
use dicom_core::value::DicomDateTime;
use dicom_core::Tag;
use std::path::PathBuf;
use std::str::FromStr;

use crate::app::types::Matcher;
use crate::app::utils::{parse_date, parse_datetime, parse_datetime_utc, parse_tag};

#[derive(Debug)]
pub struct AnonymizerValues {
    pub(crate) input: PathBuf,
    pub(crate) output: Option<PathBuf>,
    pub(crate) patient_name: Option<String>,
    pub(crate) patient_sex: Option<PatientSex>,
    pub(crate) patient_birth_day: Option<DicomDateTime>,
    pub(crate) remove_tags: Option<Vec<Tag>>,
    pub(crate) dry_run: bool,
}

impl Matcher<AnonymizerValues, AnonymizerMeta> for AnonymizerValues {
    fn match_args(matches: ArgMatches) -> Result<AnonymizerValues> {
        let dry_run = match matches.value_of("dry_run") {
            None => false,
            Some(value) => value.parse().unwrap_or(false),
        };

        let input = PathBuf::from(matches.value_of("input").unwrap());
        let output = matches.value_of("output").map(PathBuf::from);

        let patient_name = matches.value_of("patient_name").map(str::to_string);
        let patient_sex = match matches.value_of("patient_sex") {
            None => None,
            Some(v) => Some(PatientSex::from_str(v)?),
        };
        let patient_birth_day = match matches.value_of("patient_birth_day") {
            None => None,
            Some(pbd) => {
                let dt_offset = parse_datetime_utc(pbd)?;
                Some(DicomDateTime::try_from(&dt_offset)?)
            }
        };
        let remove_tags = match matches.values_of("remove_tags") {
            None => None,
            Some(rt) => {
                let mut remove_tags = Vec::<Tag>::new();
                for item in rt {
                    remove_tags.push(parse_tag(item)?);
                }

                Some(remove_tags)
            }
        };

        Ok(AnonymizerValues {
            input,
            output,
            patient_name,
            patient_sex,
            patient_birth_day,
            remove_tags,
            dry_run,
        })
    }

    fn match_trait(&self) -> Result<AnonymizerMeta> {
        let mut builder = Anonymizer::meta_builder();

        match &self.patient_name {
            Some(v) => {
                builder.patient_name(v);
            }
            None => (),
        }

        match &self.patient_sex {
            Some(ps) => {
                builder.patient_sex(ps.to_owned());
            }
            None => (),
        };

        match &self.patient_birth_day {
            Some(pbd) => {
                builder.patient_birth_date(CustomDicomDateTime::from(pbd.to_owned()));
            }
            None => (),
        };

        match &self.remove_tags {
            Some(tags) => {
                builder.remove_tags(tags.to_owned().into());
            }
            None => (),
        };

        Ok(builder.build()?)
    }
}
