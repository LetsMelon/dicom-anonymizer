use clap::{Arg, Command};

use crate::app::args::*;
use crate::app::types::StaticCommand;

pub fn anonymizer() -> StaticCommand {
    Command::new("anonymizer").args(&[
        dry_run(),
        input(),
        output(),
        patient_name(),
        patient_sex(),
        patient_birth_day(),
        remove_tags(),
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
            Arg::new("read")
                .takes_value(false)
                .short('m')
                .long("modify"),
        ])
}
