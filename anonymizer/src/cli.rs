use std::ffi::OsString;
use clap::{Arg, Command};
use crate::validator::{validator_is_date, validator_is_dcm_path};

#[derive(Debug)]
pub struct App { }

impl App {
    pub fn new() -> Self {
        Self::new_from(std::env::args_os().into_iter()).unwrap_or_else(|e| e.exit())
    }

    pub fn new_from<I, T>(args: I) -> Result<Self, clap::Error>
        where
            I: Iterator<Item = T>,
            T: Into<OsString> + Clone,
    {
        let app = Self::build_cli();

        let matches = app.get_matches_from(args);

        // TODO: parse inputs
        println!("{:?}", matches);

        Ok(App {})
    }

    fn build_cli() -> Command<'static> {
        let app = Command::new("anonymizer")
            .version("0.1.0")
            .author("Domenic Melcher")
            .arg_required_else_help(true)
            .args(&[
                Arg::new("dry_run")
                    .takes_value(false)
                    .short('d')
                    .long("dry-run")
                    .help("If set then the file will not be saved"),
                Arg::new("input")
                    .takes_value(true)
                    .value_name("FILE")
                    .required(true)
                    .help("DICOM file to anonymize")
                    .validator(validator_is_dcm_path),
                Arg::new("output")
                    .takes_value(true)
                    .short('o')
                    .long("output")
                    .help("Output path for DICOM file")
                    .validator(validator_is_dcm_path),
                Arg::new("patient_name")
                    .takes_value(true)
                    .short('p')
                    .long("patient-name")
                    .multiple_values(true)
                    .help("Change the patient name"),
                Arg::new("patient_sex")
                    .takes_value(true)
                    .long("patient-sex")
                    .help("Change the patient sex (M,F,O)"),
                Arg::new("patient_birth_day")
                    .takes_value(true)
                    .long("patient-birth-day")
                    .aliases(&["patient-bd", "patient-birthday"])
                    .help("Change the patient birthday (yyy-mm-dd or yyyy-m-d)")
                    .validator(validator_is_date),
                Arg::new("remove_tags")
                    .takes_value(true)
                    .multiple_values(true)
                    .value_delimiter(',')
                    .long("remove-tags")
                    .help("Remove dicom tags from the object. Example: 0x0010-0x0020,0x0010-0x0040"),
            ])
            ;

        app
    }
}
