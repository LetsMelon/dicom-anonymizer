use anyhow::{bail, Result};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, ParseResult, Utc};
use dicom_core::Tag;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

const DICM_PREAMBLE: [u8; 128] = [0u8; 128];
const DICM_MAGIC_CODE: [u8; 4] = [b'D', b'I', b'C', b'M'];

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
    assert_eq!(buf, DICM_PREAMBLE, "First 128 bytes should be 0x0");

    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    assert_eq!(buf, DICM_MAGIC_CODE, "Magic number should be DICM");

    Ok(true)
}

pub fn is_dcm_path<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    matches!(
        path.as_ref().extension().map(|ext| ext.to_str()),
        Some(Some("dcm"))
    )
}

pub fn parse_date(value: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
}

pub fn parse_datetime(value: &str) -> ParseResult<NaiveDateTime> {
    parse_date(value).map(|value| value.and_time(NaiveTime::from_hms(0, 0, 0)))
}

pub fn parse_datetime_utc(value: &str) -> ParseResult<DateTime<FixedOffset>> {
    let ndt = parse_datetime(value)?;
    let dtf: DateTime<FixedOffset> = DateTime::<Utc>::from_utc(ndt, Utc).into();
    Ok(dtf)
}

pub fn parse_tag(value: &str) -> Result<Tag> {
    let splitted = value.split('-').collect::<Vec<&str>>();

    if splitted.len() != 2 {
        bail!(
            "Error while parsing tags, has to be 0x____-0x____ but received {}",
            value
        );
    }

    let group_number = match u16::from_str_radix(splitted[0].trim_start_matches("0x"), 16) {
        Ok(value) => value,
        Err(_) => {
            bail!("Error while parsing input as hex number")
        }
    };
    let element_number = match u16::from_str_radix(splitted[1].trim_start_matches("0x"), 16) {
        Ok(value) => value,
        Err(_) => {
            bail!("Error while parsing input as hex number")
        }
    };

    Ok(Tag(group_number, element_number))
}

#[cfg(test)]
mod tests {
    mod is_dcm_path {
        use crate::app::utils::is_dcm_path;
        use std::path::PathBuf;

        #[test]
        fn only_allow_paths_with_dcm_extension() {
            assert!(is_dcm_path(PathBuf::from("/test.dcm")));
            assert!(is_dcm_path(PathBuf::from("./test.dcm")));
            assert!(!is_dcm_path(PathBuf::from("/test.jpg")));
            assert!(!is_dcm_path(PathBuf::from("/test")));
        }
    }

    mod parse_date {
        use crate::app::utils::parse_date;
        use chrono::NaiveDate;

        #[test]
        fn only_parse_dates_ymd_format() {
            match parse_date("2022-03-15") {
                Ok(value) => {
                    assert_eq!(value, NaiveDate::from_ymd(2022, 3, 15));
                }
                Err(_) => {
                    assert!(false)
                }
            };

            match parse_date("2022-15-03") {
                Ok(_) => {
                    assert!(false);
                }
                Err(_) => {
                    assert!(true)
                }
            };
        }
    }

    mod parse_datetime {
        use crate::app::utils::parse_datetime;
        use chrono::{NaiveDate, NaiveTime};

        #[test]
        fn only_parse_dates_ymd_format() {
            match parse_datetime("2022-03-15") {
                Ok(value) => {
                    assert_eq!(
                        value,
                        NaiveDate::from_ymd(2022, 3, 15).and_time(NaiveTime::from_hms(0, 0, 0))
                    );
                }
                Err(_) => {
                    assert!(false)
                }
            };

            match parse_datetime("2022-15-03") {
                Ok(_) => {
                    assert!(false);
                }
                Err(_) => {
                    assert!(true)
                }
            };
        }
    }

    mod parse_tag {
        use crate::app::utils::parse_tag;
        use dicom_core::Tag;

        #[test]
        fn parse_tags_in_the_right_format() {
            match parse_tag("0x0001-0x0001") {
                Ok(tag) => assert_eq!(tag, Tag(1, 1)),
                Err(_) => assert!(false),
            };
            match parse_tag("0x00FF-0x0012") {
                Ok(tag) => assert_eq!(tag, Tag(255, 18)),
                Err(_) => assert!(false),
            };
        }

        #[test]
        fn parse_errors() {
            match parse_tag("0x0001-0x0001-0x001") {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(err.to_string(), "Error while parsing tags, has to be 0x____-0x____ but received 0x0001-0x0001-0x001");
                }
            };
            match parse_tag("0x001G-0x001") {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(err.to_string(), "Error while parsing input as hex number");
                }
            };
            match parse_tag("0x001F-0x001P") {
                Ok(_) => assert!(false),
                Err(err) => {
                    assert_eq!(err.to_string(), "Error while parsing input as hex number");
                }
            };
        }
    }
}
