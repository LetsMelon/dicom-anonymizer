use clap::{Arg, Command};

use crate::app::args::anonymizer;
use crate::app::types::StaticCommand;

pub fn anonymizer() -> StaticCommand {
    Command::new("anonymizer").args(&[
        anonymizer::dry_run(),
        anonymizer::input(),
        anonymizer::output(),
        anonymizer::patient_name(),
        anonymizer::patient_sex(),
        anonymizer::patient_birth_day(),
        anonymizer::remove_tags(),
    ])
}

pub fn config() -> StaticCommand {
    Command::new("config")
        .about("Create and modify config files")
        .args(&[
            Arg::new("create")
                .takes_value(false)
                .short('c')
                .long("create"),
            Arg::new("modify")
                .takes_value(false)
                .short('m')
                .long("modify"),
            Arg::new("read").takes_value(false).short('r').long("read"),
        ])
}
