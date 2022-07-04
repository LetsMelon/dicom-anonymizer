use crate::app::types::StaticArg;

mod anonymizer;

pub fn anonymizer() -> [StaticArg; 8] {
    [
        anonymizer::dry_run(),
        anonymizer::input(),
        anonymizer::output(),
        anonymizer::patient_name(),
        anonymizer::patient_sex(),
        anonymizer::patient_birth_day(),
        anonymizer::remove_tags(),
        anonymizer::config(),
    ]
}
