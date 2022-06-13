use clap::Command;

use crate::app::args::*;

type StaticCommand = Command<'static>;

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
