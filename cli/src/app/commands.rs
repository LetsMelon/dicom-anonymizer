use clap::Command;

use crate::app::args::*;
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
        .args(&[config::create(), config::modify(), config::read()])
}
