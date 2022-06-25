use anyhow::Result;
use clap::Command;
use std::ffi::OsString;

use crate::app::anonymizer;
use crate::app::args;
use crate::app::types::StaticCommand;

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

        anonymizer::logic(matches)?;

        Ok(())
    }

    fn build_cli() -> StaticCommand {
        let args = args::anonymizer();

        Command::new("dicom-tools")
            .bin_name("dicom-tools")
            .version("0.1.0")
            .author("Domenic Melcher")
            .arg_required_else_help(true)
            .args(&args)
    }
}
