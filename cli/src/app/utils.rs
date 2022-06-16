use anyhow::Result;
use chrono::{NaiveDate, ParseResult};
use std::ffi::OsStr;
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
    let dcm_extension = OsStr::new("dcm");
    match path.as_ref().extension() {
        Some(dcm_extension) => true,
        None => false,
        Some(_) => false,
    }
}

pub fn parse_date(value: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
}
