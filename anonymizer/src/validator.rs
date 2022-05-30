use crate::utils::{is_dcm_path, parse_date};

pub type validator_type = Result<(), String>;

pub fn validator_is_dcm_path(path: &str) -> validator_type {
    if is_dcm_path(path) {
        return Ok(())
    }
    Err(String::from("Must be a valid dicom file path"))
}

pub fn validator_is_date(value: &str) -> validator_type {
    match parse_date(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Must be in yyyy-mm-dd or yyyy-m-d format"))
    }
}
