use crate::App;
use anonymizer_lib::types::CustomDicomDateTime;
use anonymizer_lib::{Anonymizer, AnonymizerMeta};
use anyhow::Result;
use chrono::{NaiveDate, ParseResult};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn is_dcm_file<P>(path: P) -> Result<bool>
where
    P: AsRef<Path>,
{
    // Inspiration:
    // - https://docs.rs/dicom-object/0.5.0/src/dicom_object/mem.rs.html#239
    // - https://docs.rs/dicom-object/0.5.0/src/dicom_object/meta.rs.html#172
    let path = path.as_ref();
    let mut file = BufReader::new(File::open(path)?);

    let mut buf = [0u8; 128];
    file.read_exact(&mut buf)?;
    assert_eq!(buf, [0u8; 128], "First 128 bytes should be 0x0");

    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    let magic_dicom_number = [b'D', b'I', b'C', b'M'];
    assert_eq!(buf, magic_dicom_number, "Magic number should be DICM");

    Ok(true)
}

pub fn is_dcm_path<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    path.as_ref().ends_with(".dcm")
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
        }
        None => (),
    };

    match &app.patient_birth_day {
        Some(pbd) => {
            builder.patient_birth_date(CustomDicomDateTime::from(pbd.to_owned()));
        }
        None => (),
    };

    match &app.remove_tags {
        Some(tags) => {
            builder.remove_tags(tags.to_owned().into());
        }
        None => (),
    };

    Ok(builder.build()?)
}
