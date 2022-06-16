use anonymizer_lib::types::CustomDicomDateTime;
use anonymizer_lib::{Anonymizer, AnonymizerMeta, PatientSex};
use anyhow::Result;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, Utc};
use clap::{ArgMatches, Command};
use dicom_core::value::DicomDateTime;
use dicom_core::Tag;
use std::ffi::OsString;
use std::str::FromStr;

use crate::app::commands::{anonymizer, config};
use crate::app::types::StaticCommand;

type Path = std::path::PathBuf;

trait Matcher<T, Y> {
    fn match_args(matches: ArgMatches) -> Result<T>;
    fn match_trait(&self) -> Result<Y>;
}

#[derive(Debug)]
pub struct AnonymizerValues {
    pub(crate) input: Path,
    pub(crate) output: Option<Path>,
    pub(crate) patient_name: Option<String>,
    pub(crate) patient_sex: Option<PatientSex>,
    pub(crate) patient_birth_day: Option<DicomDateTime>,
    pub(crate) remove_tags: Option<Vec<Tag>>,
    pub(crate) dry_run: bool,
}

impl Matcher<AnonymizerValues, AnonymizerMeta> for AnonymizerValues {
    fn match_args(matches: ArgMatches) -> Result<AnonymizerValues> {
        let dry_run = match matches.value_of("dry_run") {
            None => false,
            Some(value) => value.parse().unwrap_or(false),
        };

        let input = Path::from(matches.value_of("input").unwrap());
        let output = matches.value_of("output").map(Path::from);

        let patient_name = matches.value_of("patient_name").map(str::to_string);
        let patient_sex = match matches.value_of("patient_sex") {
            None => None,
            Some(v) => Some(PatientSex::from_str(v)?),
        };
        let patient_birth_day = match matches.value_of("patient_birth_day") {
            None => None,
            Some(pbd) => {
                let ndt = NaiveDate::parse_from_str(&*pbd, "%Y-%m-%d")?
                    .and_time(NaiveTime::from_hms(0, 0, 0));
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

                    let group_number =
                        u16::from_str_radix(splitted[0].trim_start_matches("0x"), 16)?;
                    let element_number =
                        u16::from_str_radix(splitted[1].trim_start_matches("0x"), 16)?;
                    remove_tags.push(Tag(group_number, element_number));
                }

                Some(remove_tags)
            }
        };

        Ok(AnonymizerValues {
            input,
            output,
            patient_name,
            patient_sex,
            patient_birth_day,
            remove_tags,
            dry_run,
        })
    }

    fn match_trait(&self) -> Result<AnonymizerMeta> {
        let mut builder = Anonymizer::meta_builder();

        match &self.patient_name {
            Some(v) => {
                builder.patient_name(v);
            }
            None => (),
        }

        match &self.patient_sex {
            Some(ps) => {
                builder.patient_sex(ps.to_owned());
            }
            None => (),
        };

        match &self.patient_birth_day {
            Some(pbd) => {
                builder.patient_birth_date(CustomDicomDateTime::from(pbd.to_owned()));
            }
            None => (),
        };

        match &self.remove_tags {
            Some(tags) => {
                builder.remove_tags(tags.to_owned().into());
            }
            None => (),
        };

        Ok(builder.build()?)
    }
}

#[derive(Debug)]
pub struct App {}

impl App {
    pub fn run() {
        Self::new_from(std::env::args_os()).unwrap_or_else(|e| {
            println!("{}", e);
            std::process::exit(1);
        });
    }

    fn new_from<I, T>(args: I) -> Result<()>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let app = Self::build_cli();
        let matches = app.get_matches_from(args);

        match matches.subcommand_name() {
            None => (),
            Some("config") => {
                println!("{:?}", matches);
                todo!();
            }
            Some("anonymizer") => {
                let matches = AnonymizerValues::match_args(
                    matches.subcommand_matches("anonymizer").unwrap().clone(),
                )?;

                let mut obj = Anonymizer::from_file(&matches.input.to_string_lossy())?;
                obj.meta(matches.match_trait()?);

                obj.anonymize();

                match (matches.output, matches.dry_run) {
                    (_, true) => (),
                    (None, false) => (),
                    (Some(path), false) => {
                        obj.save(path.to_string_lossy().as_ref())?;
                    }
                }
            }
            item => unreachable!(
                "Should be unreachable because clap checks it - ({})",
                item.unwrap_or("UNKNOWN")
            ),
        }

        Ok(())
    }

    fn build_cli() -> StaticCommand {
        Command::new("dicom-tools")
            .bin_name("dicom-tools")
            .version("0.1.0")
            .author("Domenic Melcher")
            .arg_required_else_help(true)
            .subcommands([anonymizer(), config()])
    }
}
