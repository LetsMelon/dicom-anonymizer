use std::path::Path;
use std::str::FromStr;
use anonymizer_lib::PatientSex;
use crate::utils::{is_dcm_path, parse_date};

pub type ValidatorType = Result<(), String>;

pub fn validator_is_dcm_path(path: &str) -> ValidatorType {
    if is_dcm_path(path) {
        return Ok(())
    }
    Err(String::from("Must be a valid dicom file path"))
}

pub fn validator_is_file_path(path: &str) -> ValidatorType {
    if Path::new(path).exists() {
        return Ok(())
    }
    Err(String::from("Must be a valid file path"))
}

pub fn validator_is_date(value: &str) -> ValidatorType {
    match parse_date(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Must be in yyyy-mm-dd or yyyy-m-d format"))
    }
}

pub fn validator_is_sex(value: &str) -> ValidatorType {
    match PatientSex::from_str(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Only one of the values M,F,O are allowed (not case sensitive)"))
    }
}
