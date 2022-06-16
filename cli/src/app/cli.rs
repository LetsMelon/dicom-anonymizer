use anonymizer_lib::Anonymizer;
use anyhow::Result;
use clap::Command;
use std::ffi::OsString;

use crate::app::anonymizer::values::AnonymizerValues;
use crate::app::commands::{anonymizer, config};
use crate::app::types::{Matcher, StaticCommand};

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
