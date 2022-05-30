use std::u16;
use dicom_core::chrono::FixedOffset;
use dicom_core::Tag;
use dicom_core::value::DicomDateTime;
use anonymizer_lib::{Anonymizer, PatientSex};
use anyhow::Result;
use clap::Parser;
use std::str::FromStr;
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use crate::cli::App;

mod cli;
mod utils;
mod validator;

fn main() -> Result<()> {
    let app = App::new();

    let mut obj = Anonymizer::from_file(&app.input.to_string_lossy())?;
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
            builder.patient_birth_date(pbd.to_owned());
        },
        None => (),
    };

    match &app.remove_tags {
        Some(tags) => {
            builder.remove_tags(tags.to_owned().into());
        },
        None => (),
    };

    obj.meta(builder.build()?);

    obj.anonymize();

    match (app.output, app.dry_run) {
        (_, true) => (),
        (None, false) => (),
        (Some(path), false) => {
            obj.save(path.to_string_lossy().as_ref())?;
        },
        (_, _) => unreachable!(),
    }

    Ok(())
}
