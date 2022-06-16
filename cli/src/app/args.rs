use clap::Arg;

use crate::app::types::StaticArg;
use crate::app::validator::{
    validator_is_date, validator_is_dcm_file, validator_is_dcm_path, validator_is_file_path,
    validator_is_sex,
};

#[inline(always)]
pub fn dry_run() -> StaticArg {
    Arg::new("dry_run")
        .takes_value(false)
        .short('d')
        .long("dry-run")
        .help("If set then the file will not be saved")
}

#[inline(always)]
pub fn input() -> StaticArg {
    Arg::new("input")
        .takes_value(true)
        .value_name("FILE")
        .required(true)
        .help("DICOM file to anonymize")
        .validator(|v| -> Result<(), String> {
            let v_df = validator_is_dcm_file(v);

            match v_df {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        })
}

#[inline(always)]
pub fn output() -> StaticArg {
    Arg::new("output")
        .takes_value(true)
        .short('o')
        .long("output")
        .help("Output path for DICOM file")
        .validator(validator_is_dcm_path)
}

#[inline(always)]
pub fn patient_name() -> StaticArg {
    Arg::new("patient_name")
        .takes_value(true)
        .short('p')
        .long("patient-name")
        .help("Change the patient name")
}

#[inline(always)]
pub fn patient_sex() -> StaticArg {
    Arg::new("patient_sex")
        .takes_value(true)
        .long("patient-sex")
        .help("Change the patient sex (M,F,O)")
        .validator(validator_is_sex)
}

#[inline(always)]
pub fn patient_birth_day() -> StaticArg {
    Arg::new("patient_birth_day")
        .takes_value(true)
        .long("patient-birth-day")
        .aliases(&["patient-bd", "patient-birthday"])
        .help("Change the patient birthday (yyy-mm-dd or yyyy-m-d)")
        .validator(validator_is_date)
}

#[inline(always)]
pub fn remove_tags() -> StaticArg {
    Arg::new("remove_tags")
        .takes_value(true)
        .multiple_values(true)
        .value_delimiter(',')
        .long("remove-tags")
        .help("Remove dicom tags from the object. Example: 0x0010-0x0020,0x0010-0x0040")
}
