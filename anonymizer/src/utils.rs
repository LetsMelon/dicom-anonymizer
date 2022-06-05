use chrono::{NaiveDate, ParseResult};
use crate::App;
use anyhow::Result;
use anonymizer_lib::{Anonymizer, AnonymizerMeta};
use anonymizer_lib::types::CustomDicomDateTime;

pub fn is_dcm_path(path: &str) -> bool {
    path.ends_with(".dcm")
}

pub fn parse_date(value: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
}

pub fn match_args_into_trait(app: &App) -> Result<AnonymizerMeta> {
    let mut builder = Anonymizer::meta_builder();

    match &app.patient_name {
        Some(v) => {
            builder.patient_name(v);
        }
        None => (),
    }

    match &app.patient_sex {
        Some(ps) => {
            builder.patient_sex(ps.to_owned());
        },
        None => (),
    };

    match &app.patient_birth_day {
        Some(pbd) => {
            builder.patient_birth_date(CustomDicomDateTime::from(pbd.to_owned()));
        },
        None => (),
    };

    match &app.remove_tags {
        Some(tags) => {
            builder.remove_tags(tags.to_owned().into());
        },
        None => (),
    };

    Ok(builder.build()?)
}
