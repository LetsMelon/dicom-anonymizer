use crate::app::utils::{is_dcm_file, is_dcm_path, parse_date};
use anonymizer_lib::PatientSex;
use std::path::Path;
use std::str::FromStr;

pub type ValidatorType = Result<(), String>;

pub fn validator_is_file_path(path: &str) -> ValidatorType {
    let path = Path::new(path);
    let file_exists = path.exists();
    let parent_exists = path.parent().map(|item| item.exists());

    match (file_exists, parent_exists) {
        (_, None) => Err(String::from("Parent directory doesn't exists")),
        (_, Some(false)) => Err(String::from("Parent directory doesn't exists")),
        (_, Some(true)) => Ok(()),
    }
}

pub fn validator_is_dcm_path(path: &str) -> ValidatorType {
    match validator_is_file_path(path) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }

    if is_dcm_path(path) {
        return Ok(());
    }
    Err(String::from("Must be a valid dicom file path"))
}

pub fn validator_is_dcm_file(path: &str) -> ValidatorType {
    match is_dcm_file(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn validator_is_date(value: &str) -> ValidatorType {
    match parse_date(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Must be in yyyy-mm-dd or yyyy-m-d format")),
    }
}

pub fn validator_is_sex(value: &str) -> ValidatorType {
    match PatientSex::from_str(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from(
            "Only one of the values M,F,O are allowed (not case sensitive)",
        )),
    }
}
