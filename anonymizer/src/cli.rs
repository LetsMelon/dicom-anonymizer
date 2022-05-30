use std::ffi::OsString;
use std::str::FromStr;
use clap::{Arg, ArgMatches, Command};
use anyhow::Result;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, Utc};
use dicom_core::Tag;
use dicom_core::value::DicomDateTime;
use anonymizer_lib::PatientSex;

use crate::validator::{validator_is_date, validator_is_dcm_path, validator_is_file_path, validator_is_sex};

type Path = std::path::PathBuf;

#[derive(Debug)]
pub struct App {
    pub(crate) input: Path,
    pub(crate) output: Option<Path>,
    pub(crate) patient_name: Option<String>,
    pub(crate) patient_sex: Option<PatientSex>,
    pub(crate) patient_birth_day: Option<DicomDateTime>,
    pub(crate) remove_tags: Option<Vec<Tag>>,
    pub(crate) dry_run: bool,
}

impl App {
    pub fn new() -> Self {
        Self::new_from(std::env::args_os()).unwrap_or_else(|e| {
            println!("{}", e);
            std::process::exit(1);
        })
    }

    pub fn new_from<I, T>(args: I) -> Result<App>
        where
            I: Iterator<Item = T>,
            T: Into<OsString> + Clone,
    {
        let app = Self::build_cli();

        let matches = app.get_matches_from(args);

        Self::match_args(matches)
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
                    .validator(|v| -> Result<(), String> {
                        let v1 = validator_is_dcm_path(v);
                        let v2 = validator_is_file_path(v);

                        match (v1, v2) {
                            (Ok(_), Ok(_)) => Ok(()),
                            (Err(s), Ok(_)) => Err(s),
                            (Ok(_), Err(s)) => Err(s),
                            (Err(s1), Err(s2)) => {
                                Err(format!("{0}, {1}", s1, s2))
                            }
                        }
                    }),
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
                    .help("Change the patient name"),
                Arg::new("patient_sex")
                    .takes_value(true)
                    .long("patient-sex")
                    .help("Change the patient sex (M,F,O)")
                    .validator(validator_is_sex),
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
            ]);

        app
    }

    pub fn match_args(matches: ArgMatches) -> Result<App> {

        println!("{:?}", matches);

        let dry_run = match matches.value_of("dry_run") {
            None => false,
            Some(value) => {
                value.parse().unwrap_or(false)
            }
        };

        let input = Path::from(matches.value_of("input").unwrap());
        let output = matches.value_of("output").map(Path::from);

        let patient_name = matches.value_of("patient_name").map(str::to_string);
        let patient_sex = match matches.value_of("patient_sex") {
            None => None,
            Some(v) => {
                Some(PatientSex::from_str(v)?)
            }
        };
        let patient_birth_day = match matches.value_of("patient_birth_day") {
            None => None,
            Some(pbd) => {
                let ndt = NaiveDate::parse_from_str(&*pbd, "%Y-%m-%d")?
                    .and_time(NaiveTime::from_hms(0, 0,0));
                let dt_offset: DateTime<FixedOffset> = DateTime::<Utc>::from_utc(ndt, Utc).into();
                Some(DicomDateTime::try_from(&dt_offset)?)
            }
        };
        let remove_tags = match matches.values_of("remove_tags") {
            None => None,
            Some(rt) => {
                let mut remove_tags = Vec::<Tag>::new();
                for item in rt {
                    let splitted = item.split('-').collect::<Vec<&str>>();

                    let group_number = u16::from_str_radix( splitted[0].trim_start_matches("0x"), 16)?;
                    let element_number = u16::from_str_radix( splitted[1].trim_start_matches("0x"), 16)?;
                    remove_tags.push(Tag {
                        0: group_number,
                        1: element_number,
                    });
                }

                Some(remove_tags)
            }
        };

        Ok(App {
            input,
            output,
            patient_name,
            patient_sex,
            patient_birth_day,
            remove_tags,
            dry_run,
        })
    }
}
